# Example Post and Message Types

Below is the output of `cable/examples/types.rs`.

**Run the example code.**

`cargo run --example types`

```
text post: { public_key: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d", signature: "1ebd8a0880f53a2caeb199ee937d056c62fde63cac91d8b30b3254441b97a1a4e4a1b45ce2372ced55752a6ce660248728b4ae40ba572ebc7cf571c43990d109", links: ["fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad86"], post_type: 0, timestamp: 9876543210, channel: "bike_life", text: "もしもし" }
text post binary: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d1ebd8a0880f53a2caeb199ee937d056c62fde63cac91d8b30b3254441b97a1a4e4a1b45ce2372ced55752a6ce660248728b4ae40ba572ebc7cf571c43990d10901fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8600eaadc0e5240962696b655f6c6966650ce38282e38197e38282e38197"

delete post: { public_key: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d", signature: "4e140fbd51d29e19e2040db36b133706107d643723f6ed3cab5ec14cc02a52faf1d2e58faf73106c2ed8fd4bc1c5aab2dc5c679efcf2f386f20c86ff7250910a", links: ["fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad86"], post_type: 1, timestamp: 9876543210, hashes: ["fcd7c41883c3564c5a6abec78e214159efe62d50f124b4afafc184ea3b764cd4", "46b321c236880cd861dafae3040cf8cc52990516d1a69ab2c170b1e615a7ebd5", "ffe809405a3e1eaf77938bde2138832b177a51e47df02935edc12aacf8279f61"] }
delete post binary: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d4e140fbd51d29e19e2040db36b133706107d643723f6ed3cab5ec14cc02a52faf1d2e58faf73106c2ed8fd4bc1c5aab2dc5c679efcf2f386f20c86ff7250910a01fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8601eaadc0e52403fcd7c41883c3564c5a6abec78e214159efe62d50f124b4afafc184ea3b764cd446b321c236880cd861dafae3040cf8cc52990516d1a69ab2c170b1e615a7ebd5ffe809405a3e1eaf77938bde2138832b177a51e47df02935edc12aacf8279f61"

info post: { public_key: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d", signature: "29090964b13c6650465c415bd1786283d3c9e0da5c1694a1542c112861bd22567fd188693703ad25e091053107e20979fb6fe04bb7473fda397fdbddd616a70a", links: ["fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad86"], post_type: 2, timestamp: 9876543210, info: [key: "name", val: "ripple"] }
info post binary: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d29090964b13c6650465c415bd1786283d3c9e0da5c1694a1542c112861bd22567fd188693703ad25e091053107e20979fb6fe04bb7473fda397fdbddd616a70a01fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8602eaadc0e524046e616d6506726970706c6500"

topic post: { public_key: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d", signature: "7245247a2965d415792cedb22e0222cef3ab9d8d174dd38c5bd26b5e0bf8ab528804f736101519121dd062d06cb0bb2c912ee09f50c3043930c6af4bc25d5b07", links: ["fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad86"], post_type: 3, timestamp: 9876543210, channel: "bike_life", topic: "tracklocross" }
topic post binary: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d7245247a2965d415792cedb22e0222cef3ab9d8d174dd38c5bd26b5e0bf8ab528804f736101519121dd062d06cb0bb2c912ee09f50c3043930c6af4bc25d5b0701fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8603eaadc0e5240962696b655f6c6966650c747261636b6c6f63726f7373"

join post: { public_key: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d", signature: "eb01378f6b71415190864dde4e31e96468d68a2b47f10278bdc92c01628b12d1940598a568e7c8bef25b3853add00746ce495b6e95a7b982ecdd2ab0ef7d6b05", links: ["fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad86"], post_type: 4, timestamp: 9876543210, channel: "qigong" }
join post binary: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5deb01378f6b71415190864dde4e31e96468d68a2b47f10278bdc92c01628b12d1940598a568e7c8bef25b3853add00746ce495b6e95a7b982ecdd2ab0ef7d6b0501fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8604eaadc0e524067169676f6e67"

leave post: { public_key: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d", signature: "08b442f83189401d70862434c547a1646bf48df6794ca6eb787df9b29c3f6c996332677a9250840c94ae07fb9d4ea0733a6a1646981ef7a8747664b715a75e0e", links: ["fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad86"], post_type: 5, timestamp: 9876543210, channel: "qigong" }
leave post binary: "aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d08b442f83189401d70862434c547a1646bf48df6794ca6eb787df9b29c3f6c996332677a9250840c94ae07fb9d4ea0733a6a1646981ef7a8747664b715a75e0e01fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8605eaadc0e524067169676f6e67"

post request: { msg_type: 2, circuit_id: "00000000", req_id: "04baaffb", ttl: 7, hashes: ["fcd7c41883c3564c5a6abec78e214159efe62d50f124b4afafc184ea3b764cd4", "46b321c236880cd861dafae3040cf8cc52990516d1a69ab2c170b1e615a7ebd5", "ffe809405a3e1eaf77938bde2138832b177a51e47df02935edc12aacf8279f61"] }
post request binary: "6b020000000004baaffb0703fcd7c41883c3564c5a6abec78e214159efe62d50f124b4afafc184ea3b764cd446b321c236880cd861dafae3040cf8cc52990516d1a69ab2c170b1e615a7ebd5ffe809405a3e1eaf77938bde2138832b177a51e47df02935edc12aacf8279f61"

cancel request: { msg_type: 3, circuit_id: "00000000", req_id: "04baaffb", ttl: 7, cancel_id: "31b5c9e1" }
cancel request binary: "0e030000000004baaffb0731b5c9e1"

channel time range request: { msg_type: 4, circuit_id: "00000000", req_id: "04baaffb", ttl: 7, channel: "qigong", time_start: 0, time_end: 2033, limit: 21 }
channel time range request binary: "15040000000004baaffb07067169676f6e6700f10f15"

channel state request: { msg_type: 5, circuit_id: "00000000", req_id: "04baaffb", ttl: 7, channel: "bike_life", future: 0 }
channel state request binary: "15050000000004baaffb070962696b655f6c69666500"

channel list request: { msg_type: 6, circuit_id: "00000000", req_id: "04baaffb", ttl: 7, offset: 0, limit: 21 }
channel list request binary: "0c060000000004baaffb070015"

hash response: { msg_type: 0, circuit_id: "00000000", req_id: "04baaffb", hashes: ["fcd7c41883c3564c5a6abec78e214159efe62d50f124b4afafc184ea3b764cd4", "46b321c236880cd861dafae3040cf8cc52990516d1a69ab2c170b1e615a7ebd5", "ffe809405a3e1eaf77938bde2138832b177a51e47df02935edc12aacf8279f61"] }
hash response binary: "6a000000000004baaffb03fcd7c41883c3564c5a6abec78e214159efe62d50f124b4afafc184ea3b764cd446b321c236880cd861dafae3040cf8cc52990516d1a69ab2c170b1e615a7ebd5ffe809405a3e1eaf77938bde2138832b177a51e47df02935edc12aacf8279f61"

post response: { msg_type: 1, circuit_id: "00000000", req_id: "04baaffb", posts: ["aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d1ebd8a0880f53a2caeb199ee937d056c62fde63cac91d8b30b3254441b97a1a4e4a1b45ce2372ced55752a6ce660248728b4ae40ba572ebc7cf571c43990d10901fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8600eaadc0e5240962696b655f6c6966650ce38282e38197e38282e38197"] }
post response binary: "aa01010000000004baaffb9e01aead820c67703da78dba364338d8b0d65d65c03a7e9310de87b2b36818ca1e5d1ebd8a0880f53a2caeb199ee937d056c62fde63cac91d8b30b3254441b97a1a4e4a1b45ce2372ced55752a6ce660248728b4ae40ba572ebc7cf571c43990d10901fea16c09f8aa581500fcf6ee2f6aabc59ccaa271d2a3568843930b7ff929ad8600eaadc0e5240962696b655f6c6966650ce38282e38197e38282e3819700"

channel list response: { msg_type: 7, circuit_id: "00000000", req_id: "04baaffb", channels: ["myco", "bike_life", "qigong"] }
channel list response binary: "20070000000004baaffb046d79636f0962696b655f6c696665067169676f6e6700"
```