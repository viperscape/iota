### sessions ###

#### negotiation ####

sessions are marked by a src node creating session id: random u32
session id is now used to authorize packets, but first we must send a session id to the dest by using the available empty flag bitset and empty route.
Since this is a negotiation packet, we use random data of u32; but we must encrypt this number using aes 128 from the secret key known out of band.

``` [..][0][0][rand u32, secret key -> aes 128] ```

the dest must respond for the session to be completed. response is empty flag, route other than 0, and data of the tombstone id encrypted using the decrypted session id.

``` [..][0][1][tid, rand u32 -> aes 128] ```

negotiation is now complete and a session id has been agreed upon


#### unique hmac ####

without sessions each iota packet's message-digest would be identical and cause alarm for an attacker could resend a packet untouched and trigger action on the recv end, therefore we need to make each message digest unique amongst eachother, even if the packets are otherwise identical.

hmac must be created as such:
1. create sha from an always random u32, flags, route, and finally data
2. hmac sha with session id as key
