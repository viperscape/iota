
### Spec Overview ###
See header in msg.rs for layout

Msg is packed as such:

```
[48 bytes: header]
==
8 bytes: tombstone id
32 bytes: message id (for auth and integ)
4 bytes: reserved bytes for protocol negotiation
4 bytes: precise time in ms in BE u32
==

0-1.4KB: data (arbitrary max size, theoretical is natural udp limit - header)

```

All packets must be authorized and checked for integrity
Authorization suggestion: HMAC-SHA256

Integrity variables (in order) [bytes 40-max_len]:
- Flags
- Route
- Time
- Message data

Integrity variables are securely hashe and used as seed in HMAC

Frames meant for negotiation must include a random byte(s) in data for security

Negotiation frames requiring random data:
- Bare Request
- Guarantee confirmations
- Batch confirmations

---


Flags defined (flags.rs):

Flags are used in 40th byte

```
Cmd = 0, // commands
Ping = 1,
Req = 1 << 1, // request
Res = 1 << 2, // response
Pub = 1 << 3, // publishing to an endpoint
G1  = 1 << 4, // guaranteed at least once
Bat = 1 << 5, // used for batching
Alt = 1 << 6, // alternate encoding specified
Alg = 1 << 7, // alternate hash algorithm

```
		
- Ping: Includes the request response model with no data attached (must be authorized)

- Request: Requests an endpoint, if no endpoint (b0) is specified then an endpoint listing should be provided (see CoAP spec). Endpoint is specified in following byte, by byte value

- Response: The endpoint state, or listing all endpoints

- Publish: Publishes to an endpoint

- Guaranteed(1): Basic guarantee(of at least once)

3 reserved bits: Currently only one might be in use for batching (more on this later)


41st byte provides value for corresponding flag, called 'route'


Example publish to route 53:

```
[..][1 << 3][..53 u16 as BE][..]

```