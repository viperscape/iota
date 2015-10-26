/*
Msg is packed as such:

[42 bytes: header]
==
8 bytes: tombstone id
32 bytes: message id (for auth and integ)
2 bytes: reserved bytes for protocol
==

0-4KB: data

*/

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};

use byteorder::{ByteOrder, BigEndian};

use ::{Client,MAX_LEN,Flags};


pub type Header = [u8;42];
trait Default {
    fn default() -> Self;
}
impl Default for Header {
    fn default() -> Header {
        [0;42]
    }
}

pub struct MsgBuilder<'d>(Header,&'d [u8]);
impl<'d> MsgBuilder<'d> {
    pub fn new(client: &Client, data: &'d [u8]) -> MsgBuilder<'d> {
        let mut h = Header::default();

        {
            let tid = &mut h[..8];
            BigEndian::write_u64(tid, client.tid);
        }
        {
            let mid = &mut h[8..40];
            let mut sha = Sha256::new();
            sha.input(data);
            let mut hmac = Hmac::new(sha,client.key());
            for (i,n) in hmac.result().code()[..32].iter().enumerate() {
                mid[i] = *n;
            }
        }

        MsgBuilder(h,data)
    }

    pub fn flag (mut self, flag: Flags) -> MsgBuilder<'d> {
        { let f = &mut self.0[40];
          *f = *f | flag.bits(); }
        self
    }

    pub fn route (mut self, rt: u8) -> MsgBuilder<'d> {
        { let f = &mut self.0[41];
          *f = rt; }
        self
    }
    
    pub fn build(mut self) -> Msg<'d> {
        Msg { header: self.0,
              data: self.1 } 
    }
}

// TODO: Move all of this to a packed tuple, (header,data)
pub struct Msg<'d> {
    header: Header,
    pub data: &'d [u8],
}

impl<'d> Msg<'d> {
    pub fn tid(&self) -> u64 {
        BigEndian::read_u64(&self.header[..8])
    }

    pub fn mid(&self) -> &[u8] {
        &self.header[8..40]
    }

    pub fn flags(&self) -> (Flags,u8) {
        (Flags::from_bits_truncate(self.header[40]),
         self.header[41])
    }
    
    // TODO: create an into_bytes without vec alloc
    // Order of packing matters here!!
    // we will expect to unpack for the same order
    pub fn into_vec(mut self) -> Vec<u8> {
        let mut v = self.header[..].to_vec();
        
        for n in self.data[..].iter() {
            v.push(*n);
        }

        v
    }

   /* pub fn as_bytes(&self) -> &[u8] {
        &self.header[..],
        &self.data[..]
    }*/
    
    /// expects buffer to be proper size
    pub fn from_bytes(buf: &[u8]) -> Msg {
        let mut h = Header::default();
        for (i,n) in buf[..42].iter().enumerate() {
            h[i] = *n;
        }

        Msg { header: h,
              data: &buf[42..] }
    }

    pub fn auth (client: &Client, msg: &Msg) -> bool {
        let mut sha = Sha256::new();
        sha.input(&msg.data[..]);
        let mut hmac = Hmac::new(sha,client.key());
        &msg.header[8..40] == hmac.result().code()
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
