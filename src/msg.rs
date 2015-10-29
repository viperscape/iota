/*
Msg is packed as such:

[42 bytes: header]
==
8 bytes: tombstone id
32 bytes: message id (for auth and integ)
2 bytes: reserved bytes for protocol negotiation
4 bytes: precise time in ms in BE u32
==

0-1.4KB: data

 */

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};

use clock_ticks::{precise_time_ms};

use byteorder::{ByteOrder, BigEndian};

use ::{Client,MAX_LEN,Flags,flags};


pub type Header = [u8;46];
trait Default {
    fn default() -> Self;
}
impl Default for Header {
    fn default() -> Header {
        [0;46]
    }
}

pub struct MsgBuilder<'d,'c>(Header,&'d [u8],&'c Client);
impl<'d,'c> MsgBuilder<'d,'c> {
    pub fn new(client: &'c Client, data: &'d [u8]) -> MsgBuilder<'d,'c> {
        let mut h = Header::default();

        {
            let tid = &mut h[..8];
            BigEndian::write_u64(tid, client.tid);
        }

        {
            let pt = &mut h[42..46];
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

    pub fn route (mut self, rt: u8) -> MsgBuilder<'d,'c> {
        { let f = &mut self.0[41];
          *f = rt; }
        
        self.gen_mid();
        
        self
    }

    fn gen_mid(&mut self) {
        let gmid = gen_mid(self.2,&self.0[40..46],&self.1[..]);
       
        let mid = &mut self.0[8..40]; 
        for (i,n) in gmid[..32].iter().enumerate() {
            mid[i] = *n;
        }
    }
    
    pub fn build(mut self) -> Msg<'d> {
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

    pub fn flags(&self) -> (Flags,u8) {
        (Flags::from_bits_truncate(self.header[40]),
         self.header[41])
    }

    pub fn time(&self) -> u32 {
        BigEndian::read_u32(&self.header[42..46])
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
        for (i,n) in buf[..46].iter().enumerate() {
            h[i] = *n;
        }

        Msg { header: h,
              data: &buf[46..] }
    }

    pub fn auth (client: &Client, msg: &Msg, max_dt: u16) -> bool {
        let mid = gen_mid(client,&msg.header[40..46],&msg.data[..]);
        //&msg.header[8..40] == &mid[..]
        
        // prevent timing attack with full iteration of values
        let mut same = true;
        for (i,n) in msg.header[8..40].iter().enumerate() {
            if same {
                same = (n == &mid[i]);
            }
        }

        if (precise_time_ms as u32 - msg.time()) < max_dt as u32 {
            same
        }
        else { false }
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

mod tests {
    extern crate test;
    use self::test::Bencher;
    
    use rand::random;
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use crypto::sha1::Sha1;
    use crypto::md5::Md5;
    use crypto::ghash::Ghash;
    
    use crypto::hmac::Hmac;
    use crypto::mac::{Mac};

    use clock_ticks::{precise_time_ns,precise_time_ms};

    use ::{Msg,MsgBuilder,Client,flags};

    #[test]
    fn conform() {
        let key = [random::<u8>();32];
        let d = [random::<u8>();1400];
        let p = [flags::Pub.bits(),125];
        
        let mut dp = [0;1402];
        dp[0] = flags::Pub.bits();
        dp[1] = 125;
        for (i,n) in d[..].iter().enumerate() {
            dp[i+2] = *n;
        }
        
        let mut sha = Sha256::new();
        sha.input(&p[..]);
        sha.input(&d[..]);
        let mut hmac = Hmac::new(sha,&key[..]).result();

        let mut sha = Sha256::new();
        sha.input(&dp[..]);
        let mut hmac2 = Hmac::new(sha,&key[..]).result();

        assert_eq!(hmac.code(),hmac2.code());
    }
    
    #[test]
    fn auth_ok () {
        let client = Client::blank();
        let m = MsgBuilder::new(&client,&b"hi"[..])
            .flag(flags::Pub).route(53).build();
        let t = m.into_vec();
        
        // t would be sent across wire as &t[..]
        // on recv we would recreate a message from bytes recv
        
        let m = Msg::from_bytes(&t[..]);
        assert!(Msg::auth(&client,&m, 150));
    }

    #[test]
    fn auth_ok_alg () {
        let client = Client::blank();
        let m = MsgBuilder::new(&client,&b"hi"[..])
            .flag(flags::Pub)
            .flag(flags::Alg).route(53).build();
        let t = m.into_vec();
        
        // t would be sent across wire as &t[..]
        // on recv we would recreate a message from bytes recv
        
        let m = Msg::from_bytes(&t[..]);
        assert!(Msg::auth(&client,&m, 150));
    }
    
    #[test]
    fn tamper_check () {
        let client = Client::blank();
        let m = MsgBuilder::new(&client,&b"hi"[..])
            .flag(flags::Pub).route(53).build();
        let pt = m.into_vec();

        // auth init msg
        {let m = Msg::from_bytes(&pt[..]);
         assert!(Msg::auth(&client,&m, 150));}
        let mut t = pt.clone(); // we'll test against this later
        
        // test flag tampering
        t[40] = flags::Req.bits(); //change pub to req
        {let m = Msg::from_bytes(&t[..]);
         assert!(!Msg::auth(&client,&m, 150));}
        t[40] = flags::Pub.bits(); // change back flag

        // test route tampering
        t[41] = 52; //change route destination
        {let m = Msg::from_bytes(&t[..]);
         assert!(!Msg::auth(&client,&m, 150));}
        t[41] = 53; //change back route

        // verify data tampering
        t[46] = 105; // change data to "ii" instead of "hi"
        {let m = Msg::from_bytes(&t[..]);
         assert!(!Msg::auth(&client,&m, 150));}
        t[46] = 104; // change back data
        
        // verify basic auth works
        {let m = Msg::from_bytes(&t[..]);
         assert!(Msg::auth(&client,&m, 150));}

        // compare original and latest
        assert_eq!(pt,t);
    }

    #[bench]
    fn md5(b:&mut Bencher) {
        let key = [random::<u8>();32];
        let d = [random::<u8>();1400];
        
        b.iter(||{
            let mut sha = Md5::new();
            sha.input(&d[..]);
            let mut hmac = Hmac::new(sha,&key[..]).result();
        });
    }

    #[bench]
    fn sha256(b:&mut Bencher) {
        let key = [random::<u8>();32];
        let d = [random::<u8>();1400];
        
        b.iter(||{
            let mut sha = Sha256::new();
            sha.input(&d[..]);
            let mut hmac = Hmac::new(sha,&key[..]).result();
        });
    }

    #[bench]
    fn sha1(b:&mut Bencher) {
        let key = [random::<u8>();32];
        let d = [random::<u8>();1400];
        
        b.iter(||{
            let mut sha = Sha1::new();
            sha.input(&d[..]);
            let mut hmac = Hmac::new(sha,&key[..]).result();
        });
    }

    #[bench]
    fn ghash(b:&mut Bencher) {
        let key = [random::<u8>();16];
        let d = [random::<u8>();1400];
        
        b.iter(||{
            let mut ha = Ghash::new(&key);
            ha.input(&d[..]);
            let mut gmac = ha.result();
        });
    }
}
