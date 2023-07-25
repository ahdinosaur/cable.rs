//! The cable manager module is responsible for tracking peer interactions,
//! handling request and response messages and querying and updating the store.
//! It is intended to serve as the main entrypoint for running a cable peer.

use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
};

use async_std::{
    channel,
    prelude::*,
    sync::{Arc, RwLock},
    task,
};
use cable::{
    constants::NO_CIRCUIT,
    error::Error,
    message::{Message, MessageBody, MessageHeader, RequestBody, ResponseBody},
    post::Post,
    Channel, ChannelOptions, Hash, ReqId,
};
use desert::{FromBytes, ToBytes};
use futures::io::{AsyncRead, AsyncWrite};
use length_prefixed_stream::{decode_with_options, DecodeOptions};

use crate::{store::Store, stream::PostStream};

// Define the TTL (how many times a request will be
// forwarded.
//
// NOTE: We may want to set this dynamically in the
// future, either based on user choice or connectivity
// status.
const TTL: u8 = 0;

pub type PeerId = usize;

/// The manager for a single cable instance.
#[derive(Clone)]
pub struct CableManager<S: Store> {
    /// A cable store.
    pub store: S,
    /// Peers with whom communication is underway.
    peers: Arc<RwLock<HashMap<PeerId, channel::Sender<Message>>>>,
    /// The most recently assigned peer ID.
    last_peer_id: Arc<RwLock<PeerId>>,
    /// The most recently assigned request ID.
    last_req_id: Arc<RwLock<u32>>,
    /// Active inbound requests to which the local peer is listening and
    /// responding.
    // TODO: Consider renaming `inbound_requests`, `active_requests` or
    // `remote_requests`.
    listening: Arc<RwLock<HashMap<PeerId, Vec<(ReqId, ChannelOptions)>>>>,
    /// Post hashes which have been requested from remote peers by the local peer.
    requested: Arc<RwLock<HashSet<Hash>>>,
    /// Active outbound requests (includes requests of local and remote origin).
    // TODO: Consider renaming `outbound_requests`.
    open_requests: Arc<RwLock<HashMap<ReqId, Message>>>,
}

impl<S> CableManager<S>
where
    S: Store,
{
    pub fn new(store: S) -> Self {
        Self {
            store,
            peers: Arc::new(RwLock::new(HashMap::new())),
            last_peer_id: Arc::new(RwLock::new(0)),
            last_req_id: Arc::new(RwLock::new(0)),
            listening: Arc::new(RwLock::new(HashMap::new())),
            requested: Arc::new(RwLock::new(HashSet::new())),
            open_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<S> CableManager<S>
where
    S: Store,
{
    /// Publish a new text post.
    pub async fn post_text<T: Into<String>, U: Into<String>>(
        &mut self,
        channel: T,
        text: U,
    ) -> Result<(), Error> {
        let public_key = self.get_public_key().await?;
        let channel = channel.into();
        let links = vec![self.get_link(&channel).await?];
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let text = text.into();

        // Construct a new text post.
        let post = Post::text(public_key, links, timestamp, channel, text);

        self.post(post).await
    }

    /// Publish a post.
    pub async fn post(&mut self, mut post: Post) -> Result<(), Error> {
        // Sign the post if required.
        if !post.is_signed() {
            post.sign(&self.get_secret_key().await?)?;
        }

        // Insert the post into the local store.
        self.store.insert_post(&post).await?;

        // Iterate over all peers and requests to whom we are listening.
        for (peer_id, reqs) in self.listening.read().await.iter() {
            // Iterate over peer requests.
            for (req_id, opts) in reqs {
                let limit = opts.limit.min(4096);
                let mut hashes = vec![];

                {
                    // Get all posts matching the request parameters.
                    let mut stream = self.store.get_post_hashes(opts).await?;
                    while let Some(result) = stream.next().await {
                        hashes.push(result?);
                        // Break once the request limit has been reached.
                        if hashes.len() as u64 >= limit {
                            break;
                        }
                    }
                }

                // Construct a new hash response message.
                let response = Message::hash_response(NO_CIRCUIT, *req_id, hashes);

                // Send the response to the peer.
                self.send(*peer_id, &response).await?;
            }
        }

        Ok(())
    }

    /// Broadcast a message to all peers.
    pub async fn broadcast(&self, message: &Message) -> Result<(), Error> {
        for ch in self.peers.read().await.values() {
            ch.send(message.clone()).await?;
        }
        Ok(())
    }

    /// Send a message to a single peer identified by the given peer ID.
    pub async fn send(&self, peer_id: usize, msg: &Message) -> Result<(), Error> {
        if let Some(ch) = self.peers.read().await.get(&peer_id) {
            ch.send(msg.clone()).await?;
        }
        Ok(())
    }

    /// Handle a request or response message.
    pub async fn handle(&mut self, peer_id: usize, msg: &Message) -> Result<(), Error> {
        let MessageHeader {
            msg_type: _,
            circuit_id,
            req_id,
        } = msg.header;

        // TODO: Forward requests.
        match &msg.body {
            MessageBody::Request { ttl: _, body } => match body {
                RequestBody::Post { hashes } => {
                    let posts = self.store.get_post_payloads(hashes).await?;
                    let response = Message::post_response(circuit_id, req_id, posts);

                    self.send(peer_id, &response).await?
                }
                RequestBody::Cancel { cancel_id } => {
                    // Remove the request from the list of open requests.
                    // The associated message will no longer be sent to peers.
                    self.open_requests.write().await.remove(cancel_id);

                    // TODO: Must be forwarded to all peers to whom the
                    // original request was forwarded.
                }
                RequestBody::ChannelTimeRange {
                    channel,
                    time_start,
                    time_end,
                    limit,
                } => {
                    let opts = ChannelOptions {
                        channel: channel.to_string(),
                        time_start: *time_start,
                        time_end: *time_end,
                        limit: *limit,
                    };
                    let n_limit = (*limit).min(4096);

                    let mut hashes = vec![];
                    {
                        // Create a stream of post hashes matching the given criteria.
                        let mut stream = self.store.get_post_hashes(&opts).await?;
                        // Iterate over the hashes in the stream.
                        while let Some(result) = stream.next().await {
                            hashes.push(result?);
                            // Break out of the loop once the requested limit is met.
                            if hashes.len() as u64 >= n_limit {
                                break;
                            }
                        }
                    }

                    let response = Message::hash_response(circuit_id, req_id, hashes);

                    // Add the peer and request ID to the request tracker if
                    // the end time has been set to 0 (i.e. keep this request
                    // alive and send new messages as they become available).
                    if *time_end == 0 {
                        let mut w = self.listening.write().await;
                        if let Some(listeners) = w.get_mut(&peer_id) {
                            listeners.push((req_id, opts));
                        } else {
                            w.insert(peer_id, vec![(req_id, opts)]);
                        }
                    }

                    self.send(peer_id, &response).await?;
                }
                RequestBody::ChannelState {
                    channel: _,
                    future: _,
                } => {
                    /*
                    TODO: We will require channel state indexes before this
                    handler can be completed.

                    Channel state includes (spec section 5.4.4):

                    The latest post/info post of all members and ex-members.
                    The latest of all users' post/join or post/leave posts to the channel.
                    The latest post/topic post made to the channel.
                    */
                }
                RequestBody::ChannelList { skip, limit } => {
                    let mut all_channels = self.store.get_channels().await?;
                    let n_limit = (*limit).min(4096);
                    // Drain the channels matching the given range.
                    let channels = all_channels
                        .drain(*skip as usize..n_limit as usize)
                        .collect();
                    let response = Message::channel_list_response(circuit_id, req_id, channels);

                    self.send(peer_id, &response).await?
                }
            },
            MessageBody::Response { body } => match body {
                // TODO: A responder MUST send a Hash Response message with
                // hash_count = 0 to indicate that they do not intend to return
                // any further hashes for the given req_id and they have
                // concluded the request on their side.
                ResponseBody::Hash { hashes } => {
                    let wanted_hashes = self.store.want(hashes).await?;
                    if !wanted_hashes.is_empty() {
                        // If a hash appears in our list of wanted hashed,
                        // send a request for the associated post.
                        let request = Message::post_request(
                            circuit_id,
                            req_id,
                            TTL,
                            wanted_hashes.to_owned(),
                        );

                        self.send(peer_id, &request).await?;

                        {
                            // Update the list of requested hashes.
                            let mut requested_posts = self.requested.write().await;
                            for hash in &wanted_hashes {
                                requested_posts.insert(*hash);
                            }
                        }
                    }

                    // TODO: If hash_count == 0, remove the request.
                }
                ResponseBody::Post { posts } => {
                    // Iterate over the encoded posts.
                    for post_bytes in posts {
                        // Verify the post signature.
                        if !Post::verify(post_bytes) {
                            // Skip to the next post, bypassing the rest of the
                            // code in this `for` loop.
                            continue;
                        }

                        // Deserialize the post.
                        let (s, post) = Post::from_bytes(post_bytes)?;

                        // Ensure the number of processed bytes matches the
                        // received amount.
                        if s != post_bytes.len() {
                            continue;
                        }

                        let post_hash = post.hash()?;

                        let mut requested_posts = self.requested.write().await;
                        // Check if this post was previously requested.
                        if !requested_posts.contains(&post_hash) {
                            // Skip this post if it was not requested.
                            continue;
                        }
                        // Remove the post hash from the list of requested
                        // posts.
                        requested_posts.remove(&post_hash);

                        // TODO: Hand the post over to an indexer.
                        // The indexer will be responsible for matching on
                        // the post type, extracting key info and indexing it
                        // in the store.

                        self.store.insert_post(&post).await?;
                    }
                }
                ResponseBody::ChannelList { channels } => {
                    // TODO: Do we need to take action to conclude the request
                    // which resulted in this response?
                    self.store.insert_channels(channels).await?;
                }
            },
            // Ignore unrecognized message type.
            MessageBody::Unrecognized { .. } => (),
        }

        Ok(())
    }

    /// Generate a new request ID.
    async fn new_req_id(&self) -> Result<(u32, ReqId), Error> {
        let mut last_req_id = self.last_req_id.write().await;

        // Reset request ID to 0 if the maximum u32 has been reached.
        // Otherwise, increment the last request ID by one.
        *last_req_id = if *last_req_id == u32::MAX {
            0
        } else {
            *last_req_id + 1
        };

        let req_id = *last_req_id;

        Ok((req_id, req_id.to_bytes()?.try_into().unwrap()))
    }

    /// Generate a new peer ID.
    async fn new_peer_id(&self) -> Result<usize, Error> {
        let mut last_peer_id = self.last_peer_id.write().await;

        // Increment the last peer ID.
        *last_peer_id += 1;
        let peer_id = *last_peer_id;

        Ok(peer_id)
    }

    /// Create a channel time range request matching the given channel
    /// parameters and broadcast it to all peers, listening for responses.
    pub async fn open_channel(
        &mut self,
        channel_opts: &ChannelOptions,
    ) -> Result<PostStream<'_>, Error> {
        let (_req_id, req_id_bytes) = self.new_req_id().await?;

        let request = Message::channel_time_range_request(
            NO_CIRCUIT,
            req_id_bytes,
            TTL,
            channel_opts.to_owned(),
        );

        self.open_requests
            .write()
            .await
            .insert(req_id_bytes, request.clone());

        self.broadcast(&request).await?;

        self.store.get_posts_live(channel_opts).await
    }

    pub async fn close_channel(&self, _channel: &[u8]) {
        // TODO: Cancel the channel time range request associated
        // with this channel. Might require the request ID generated in the
        // originating `open_channel()`...
        unimplemented![]
    }

    pub async fn get_peer_ids(&self) -> Vec<usize> {
        self.peers
            .read()
            .await
            .keys()
            .copied()
            .collect::<Vec<usize>>()
    }

    // TODO: Convert to `get_links()`?
    pub async fn get_link(&mut self, channel: &Channel) -> Result<Hash, Error> {
        let link = self.store.get_latest_hash(channel).await?;

        Ok(link)
    }

    /// Retrieve the public key of the local peer.
    pub async fn get_public_key(&mut self) -> Result<[u8; 32], Error> {
        let (pk, _sk) = self.store.get_or_create_keypair().await?;

        Ok(pk)
    }

    /// Retrieve the secret key of the local peer.
    pub async fn get_secret_key(&mut self) -> Result<[u8; 64], Error> {
        let (_pk, sk) = self.store.get_or_create_keypair().await?;

        Ok(sk)
    }

    /// Listen for incoming peer messages and respond with locally-generated
    /// messages.
    ///
    /// Decode each received message and pass it off to the handler.
    pub async fn listen<T>(&self, mut stream: T) -> Result<(), Error>
    where
        T: AsyncRead + AsyncWrite + Clone + Unpin + Send + Sync + 'static,
    {
        // Generate a new peer ID.
        let peer_id = self.new_peer_id().await?;

        // Create a bounded message channel.
        let (send, recv) = channel::bounded(100);

        // Insert the peer ID and channel sender into the list of peers.
        self.peers.write().await.insert(peer_id, send);

        // Write all open request messages to the stream.
        for msg in self.open_requests.read().await.values() {
            stream.write_all(&msg.to_bytes()?).await?;
        }

        let write_to_stream_res = {
            let mut stream_c = stream.clone();

            task::spawn(async move {
                // Listen for incoming locally-generated messages.
                while let Ok(msg) = recv.recv().await {
                    // Write the message to the stream.
                    stream_c.write_all(&msg.to_bytes()?).await?;
                }

                // Type inference fails without binding concretely to `Result`.
                Result::<(), Error>::Ok(())
            })
        };

        // Define the stream decoder parameters.
        let options = DecodeOptions {
            include_len: true,
            ..Default::default()
        };

        let mut length_prefixed_stream = decode_with_options(stream, options);

        // Iterate over the stream.
        while let Some(read_buf) = length_prefixed_stream.next().await {
            let buf = read_buf?;

            // Deserialize the received message.
            let (_, msg) = Message::from_bytes(&buf)?;

            let mut this = self.clone();
            task::spawn(async move {
                // Handle the received message.
                if let Err(e) = this.handle(peer_id, &msg).await {
                    // TODO: Consider a better way to report.
                    eprintln!["{}", e];
                }
            });
        }

        // Continue reading and writing to the peer stream until the stream is
        // closed (either intentionally or because of an error).
        write_to_stream_res.await?;

        // Remove the peer from the list of active peers.
        self.peers.write().await.remove(&peer_id);

        Ok(())
    }
}
