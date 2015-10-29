### Sessions ###

#### Negotiation ####

Sessions are marked by a src node creating session id: random u32

Session id is now used to authorize packets, but first we must send a session id to the dest by using the available empty flag bitset and empty route.
Since this is a negotiation packet, we use random data of u32; but we must encrypt this number using aes 128 from the secret key known out of ban, salted with the precise time which is sent along in the packet header.

``` [..][0][0][time u32][rand u32, secret key + time -> aes 128] ```

The dest must respond for the session to be completed. response is empty flag, route 1, and data of the tombstone id encrypted using the decrypted session id, and finally salted with new time.

``` [..][0][1][new time u32][tid, rand u32 + time -> aes 128] ```

Negotiation is now complete and a session id has been agreed upon.


#### Unique hmac ####

Between sessions and time comparison, an attacker will not be able to resend a packet untouched to trigger action on the recv end. For this to work we need to make each message digest unique amongst eachother, even if the packets are otherwise identical. As well we need to set a max delta time for stale packets to arrive out of order. If a packet arrives with a delta time which is too stale then it is discarded.

hmac must be created as such:
1. create sha from a precise time ms/u32, flags, route, and finally data
2. hmac sha with session id as key
3. precise time is used to create delta time from time sent in header
4. sha is rebuilt on dest for auth/integ checks
5. delta time comparison must be less than threshold
NOTE: steps 4 and 5 might be best in other order
