use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};

use ::Client;

pub struct Msg {
    pub mid: Vec<u8>, //short term, message id, always changes
    pub data: Vec<u8>,
}

impl Msg {
    pub fn new (client: &Client, data: &[u8]) -> Msg {
        //let dt = precise_time_ns() - client.et;
        let mut sha = Sha256::new();
        sha.input(data);
        let mut hmac = Hmac::new(sha,client.key());
        
        Msg {
            data: data.to_vec(),
            mid: hmac.result().code().to_vec(),
        }
    }
    
    pub fn into_vec(mut self) -> Vec<u8> {
        let mut v = self.mid;
        for n in self.data.drain(..) {
            v.push(n);
        }

        v
    }

    pub fn from_bytes(buf: &[u8]) -> Msg {
        let mid = &buf[..32];
        let data = &buf[32..];

        Msg { mid: mid.to_vec(),
              data: data.to_vec(),
        }
    }

    pub fn auth (client: &Client, msg: &Msg) -> bool {
        let mut sha = Sha256::new();
        sha.input(&msg.data[..]);
        let mut hmac = Hmac::new(sha,client.key());
        &msg.mid[..] == hmac.result().code()
    }
}
