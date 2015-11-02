
#### batch model ####

- batch req sent to endpoint (batch | req, route/u16) // starts
- confirm: batch res sent to src (batch | res, route/u16)
- batch req sent to endpoint (batch | req, total/u8)
- confirm: batch res sent to src (batch | res, total/u8)
- batch req sent to endpoint (batch | req, current/u8) + data

if newer packets are found, immeditately ask for missing packet
confirm: batch res sent to src (batch | res, needed/u8)

```
loop {
  batch req sent to endpoint (batch | req, route/u16) // ends

  endpoint then asks for resend if needed
  confirm: batch res sent to src (batch | res, needed/u8)
  }
```
---

#### guarantee ####

- src: store mid to be checked against on response from dest
- G1 req with route sent to endpoint (g1 | req | rt, route/u16) + data
- G1 response to src (g1 | resp) + data (mid from g1-req)
- src: confirm mids match
