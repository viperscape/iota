//use clock_ticks::precise_time_ns;
use rand::random;

#[derive(Debug)]
pub struct Client {
    tid: u64, //long term (tombstone) client id, never changes
    et: u64, // epoch time of initial connection
    key: Vec<u8>, //shared key
}

impl Client {
    pub fn blank() -> Client {
        let mut k = vec!();
        for _ in (0..64) { k.push(random::<u8>()); }

        Client {
            tid: 0,
            et: 0,
            key: k,
        }
    }

    pub fn key(&self) -> &[u8] {
        &self.key[..]
    }
}
