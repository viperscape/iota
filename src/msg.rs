/*
Msg is packed as such:

[48 bytes: header]
==
8 bytes: tombstone id
32 bytes: message id (for auth and integ)
4 bytes: reserved bytes for protocol negotiation
4 bytes: precise time in ms in BE u32
==

0-1.4KB: data

 */

#![allow(unused_imports)]

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};

use clock_ticks::{precise_time_ms};

use byteorder::{ByteOrder, BigEndian};

use ::{Client,MAX_DATA,Flags,flags};

#[allow(non_upper_case_globals)]
pub const HeaderSize: usize = 48;

pub type Header = [u8;HeaderSize];
trait Default {
    fn default() -> Self;
}
impl Default for Header {
    fn default() -> Header {
        [0;HeaderSize]
    }
}

pub struct MsgBuilder<'d,'c>(pub Header,&'d [u8],&'c Client);
impl<'d,'c> MsgBuilder<'d,'c> {
    pub fn new(client: &'c Client, data: &'d [u8]) -> MsgBuilder<'d,'c> {
        let mut h = Header::default();

        {
            let tid = &mut h[..8];
            BigEndian::write_u64(tid, client.tid);
        }

        {
            let pt = &mut h[44..48];
            BigEndian::write_u32(pt, precise_time_ms as u32);
        }

        let mut mb = MsgBuilder(h,data, client);
        mb.gen_mid();

        mb
    }

    pub fn flag (mut self, flag: Flags) -> MsgBuilder<'d,'c> {
        { let f = &mut self.0[40];
          *f = *f | flag.bits(); }

        self.gen_mid();
        
        self
    }

    pub fn route (mut self, rt: u16) -> MsgBuilder<'d,'c> {
        {let mut nrt = &mut self.0[41..43];
         BigEndian::write_u16(nrt, rt);}
        
        self.gen_mid();
        
        self
    }

    fn gen_mid(&mut self) {
        let gmid = gen_mid(self.2,&self.0[40..48],&self.1[..]);
       
        let mid = &mut self.0[8..40]; 
        for (i,n) in gmid[..32].iter().enumerate() {
            mid[i] = *n;
        }
    }
    
    pub fn build(self) -> Msg<'d> {
        Msg { header: self.0,
              data: self.1 } 
    }
}

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

    pub fn flags(&self) -> (Flags,u16) {
        (Flags::from_bits_truncate(self.header[40]),
         BigEndian::read_u16(&self.header[41..43])) //41 and 41 for route as u16
    }

    pub fn time(&self) -> u32 {
        BigEndian::read_u32(&self.header[44..48])
    }
    
    // Order of packing matters here!!
    // we will expect to unpack for the same order
    pub fn into_vec(self) -> Vec<u8> {
        let mut v = self.header[..].to_vec();
        
        for n in self.data[..].iter() {
            v.push(*n);
        }

        v
    }

    /// write to existing buffer, returns length written
    pub fn into_buf(self, buf: &mut [u8]) -> usize {
        let hlen = self.header.len();
        let dlen = self.data.len();

        if buf.len() < hlen+dlen { return 0 }
        
        for (i,n) in self.header[..].iter().enumerate() {
            buf[i] = *n;
        }

        for (i,n) in self.data[..].iter().enumerate() {
            buf[i+hlen-1] = *n;
        }

        hlen + dlen
    }
    
    /// expects buffer to be proper size
    pub fn from_bytes(buf: &[u8]) -> Msg {
        let mut h = Header::default();
        for (i,n) in buf[..48].iter().enumerate() {
            h[i] = *n;
        }

        Msg { header: h,
              data: &buf[48..] }
    }

    pub fn auth (client: &Client, msg: &Msg, max_dt: u16) -> bool {
        let mid = gen_mid(client,&msg.header[40..48],&msg.data[..]);
       
        //&msg.header[8..40] == &mid[..]
        
        // prevent timing attack with full iteration of values
        let mut same = true;
        for (i,n) in msg.header[8..40].iter().enumerate() {
            if same {
                same = n == &mid[i];
            }
        }

        if precise_time_ms as u32 >= msg.time() {
            let dt = precise_time_ms as u32 - msg.time();
            if dt < max_dt as u32 {
                return same
            }
        }

        false
    }
}

pub fn gen_mid (client: &Client, h: &[u8], d: &[u8]) -> [u8;32] {
    let hmac = {
        if Flags::from_bits_truncate(h[0])
            .contains(flags::Alg) {
                let mut sha = Sha1::new();
                sha.input(&h[..]);
                sha.input(&d[..]);
                Hmac::new(sha,client.key()).result()
            }
        else {
            let mut sha = Sha256::new();
            sha.input(&h[..]);
            sha.input(&d[..]);
            Hmac::new(sha,client.key()).result()
        }
    };

    
    let mut mid = [0u8;32]; //sha1 gets padded with zero
    for (i,n) in hmac.code()[..].iter().enumerate() {
        mid[i] = *n;
    }
    
    mid
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
