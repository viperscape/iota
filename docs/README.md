
### Spec Overview ###
See header in msg.rs for layout

Msg is packed as such:

```
[42 bytes: header]
==
8 bytes: tombstone id
32 bytes: message id (for auth and integ)
2 bytes: reserved bytes for protocol negotiation
==

0-1.4KB: data (arbitrary max size, theoretical is natural udp limit - header)

```

All packets must be authorized and checked for integrity
Authorization: HMAC-SHA256
Integrity: Message data is securely hashed (SHA256) and used as seed in HMAC
Frames meant for negotiation must include a random byte(s) in data for security

Negotiation frames requiring random data:
- Bare Request
- Guarantee confirmations
- Batch confirmations

---


Flags defined (flags.rs):

Flags are used in 41st byte

```
Ping = 1,
Req = 1 << 1, // request
Res = 1 << 2, // response
Pub = 1 << 3, // publishing to an endpoint
G1  = 1 << 4, // guaranteed at least once

// currently reserved bits for future extension
R1 = 1 << 5, // probably will be used for batching
R2 = 1 << 6,
R3 = 1 << 7,

```
		
- Ping: Includes the request response model with no data attached (must be authorized)

- Request: Requests an endpoint, if no endpoint (b0) is specified then an endpoint listing should be provided (see CoAP spec). Endpoint is specified in following byte, by byte value

- Response: The endpoint state, or listing all endpoints

- Publish: Publishes to an endpoint

- Guaranteed(1): Basic guarantee(of at least once)

3 reserved bits: Currently only one might be in use for batching (more on this later)


42nd byte provides value for corresponding flag


example publish to route 53:

```
[..][1 << 3][053][..]

```
