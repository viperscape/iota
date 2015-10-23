extern crate crypto;
extern crate clock_ticks;
extern crate rand;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::{Mac,MacResult};

use clock_ticks::precise_time_ns;

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
        for n in (0..64) { k.push(random::<u8>()); }
        
        Client {
            tid: 0,
            et: 0,
            key: k,
        }
    }
}

pub struct Msg {
    pub mid: Vec<u8>, //short term, message id, always changes
    pub data: Vec<u8>,
}

impl Msg {
    pub fn new (client: &Client, data: &[u8]) -> Msg {
        let dt = precise_time_ns() - client.et;
        
        let mut hmac = Hmac::new(Sha256::new(),&client.key[..]);
        
        Msg {
            data: data.to_vec(),
            mid: hmac.result().code().to_vec(),
        }
    }

    pub fn auth (client: &Client, msg: &Msg) -> bool {
        let mut hmac = Hmac::new(Sha256::new(),&client.key[..]);
        &msg.mid[..] == hmac.result().code()
    }
}
