
#### batch model ####

- batch req sent to endpoint (batch | req, route/u8) // starts
- confirm: batch res sent to src (batch | res, route/u8)
- batch req sent to endpoint (batch | req, total/u8)
- confirm: batch res sent to src (batch | res, total/u8)
- batch req sent to endpoint (batch | req, current/u8) + data

if newer packets are found, immeditately ask for missing packet
confirm: batch res sent to src (batch | res, needed/u8)

```
loop {
  batch req sent to endpoint (batch | req, route/u8) // ends

  endpoint then asks for resend if needed
  confirm: batch res sent to src (batch | res, needed/u8)
  }
```
---

#### guarantee ####

- src md5 hash of data to be guaranteed
- G1 req with route sent to endpoint (g1 | req | rt, route/u8) + data
- G1 response to src (g1 | res) + md5 hash of data recv
- confirm: src checks hash matches
