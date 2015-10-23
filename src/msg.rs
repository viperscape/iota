use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};

use byteorder::{ByteOrder, BigEndian};

use ::Client;

pub struct Msg {
    pub mid: [u8;32], //short term, message id, always changes
    pub data: Vec<u8>,
    pub tid: [u8;8], //client tombstone id
}

impl Msg {
    pub fn new (client: &Client, data: &[u8]) -> Msg {
        //let dt = precise_time_ns() - client.et;
        let mut sha = Sha256::new();
        sha.input(data);
        let mut hmac = Hmac::new(sha,client.key());

        let mut tid = [0; 8];
        BigEndian::write_u64(&mut tid, client.tid);
        
        Msg {
            data: data.to_vec(),
            mid: collect_u8_32(&hmac.result().code()[..32]),
            tid: tid,
        }
    }

    // TODO: create an into_bytes without vec alloc
    pub fn into_vec(mut self) -> Vec<u8> {
        let mut v = self.tid[..].to_vec();
        let mut mid = self.mid[..].to_vec();
        for n in mid.drain(..) {
            v.push(n);
        }
        for n in self.data.drain(..) {
            v.push(n);
        }

        v
    }

    pub fn from_bytes(buf: &[u8]) -> Msg {
        let mut tid = collect_u8_8(&buf[..8]);
        let mut mid = collect_u8_32(&buf[8..40]);
        let data = &buf[40..];

        Msg { tid: tid,
              mid: mid,
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

pub fn collect_u8_32 (d: &[u8]) -> [u8;32] {
    let mut v: [u8;32] = [0;32];
    for (i,n) in d.iter().enumerate() {
        v[i] = *n;
    }

    v
}

pub fn collect_u8_8 (d: &[u8]) -> [u8;8] {
    let mut v: [u8;8] = [0;8];
    for (i,n) in d.iter().enumerate() {
        v[i] = *n;
    }

    v
}
