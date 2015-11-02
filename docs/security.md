### Sessions ###

#### Negotiation ####

Sessions are marked by a src node creating session id as a random 128bit key, represented as 16 bytes.

Session id is then used to authorize packets, but first we must send a session id to the dest by using the available empty flag bitset and empty route.
We must encrypt this number using aes 128 from the secret key known out of band.

``` [..][0][0][time u32][session key, secret key -> aes 128] ```

The dest must respond for the session to be completed. Response is empty flag, route 1, and data of the same session key, encrypted.

``` [..][0][1][new time u32][session key encrypted] ```

Negotiation is now complete and a session has been agreed upon.


#### Unique hmac ####

Between sessions and time comparison, an attacker will not be able to resend a packet untouched to trigger action on the recv end. For this to work we need to make each message digest unique amongst eachother, even if the packets are otherwise identical. As well we need to set a max delta time for stale packets to arrive out of order. If a packet arrives with a delta time which is too stale then it is discarded.

hmac must be created as such:
1. src: create sha from a precise time ms/u32, flags, route, and finally data
2. src: hmac sha with session id as key and sent over wire
3. dest: precise time is used to create delta time from time sent in header
4. dest: sha is rebuilt for auth/integ checks
5. dest: delta time comparison must be less than threshold
NOTE: steps 4 and 5 might be best in reverse order
