
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

#### guarantee/rendevous model ####
  
- G1 req sent to endpoint (g1 | req)
- G1 response to src (g1 | res, rendevous-route/u8)
- req sent to endpoint (req | route, rendevous-route)
