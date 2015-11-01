#![feature(test)]

extern crate test;

#[macro_use] extern crate bitflags;
extern crate crypto;
extern crate clock_ticks;
extern crate rand;
extern crate byteorder;
extern crate iota;

use rand::random;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crypto::hmac::Hmac;
use crypto::mac::{Mac};

use byteorder::{ByteOrder, BigEndian};

use iota::{Msg,MsgBuilder,Client,flags};

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
    let hmac = Hmac::new(sha,&key[..]).result();

    let mut sha = Sha256::new();
    sha.input(&dp[..]);
    let hmac2 = Hmac::new(sha,&key[..]).result();

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
fn ping_ok () {
    use iota::comm::{ping_req,ping_resp};
    let client = Client::blank();
    let req = ping_req(&client);
    
    let r = Msg::from_bytes(&req[..]);
    let it = BigEndian::read_f32(r.data);
    
    let res = ping_resp(&client,r.data);
    let r = Msg::from_bytes(&res[..]);

    assert!(r.flags().0.contains(flags::Ping|flags::Res));
    let ot = BigEndian::read_f32(r.data);
    assert_eq!(it,ot);
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
    t[42] = 52; //change route destination
    {let m = Msg::from_bytes(&t[..]);
     assert!(!Msg::auth(&client,&m, 150));}
    t[42] = 53; //change back route

    // verify data tampering
    t[48] = 105; // change data to "ii" instead of "hi"
    {let m = Msg::from_bytes(&t[..]);
     assert!(!Msg::auth(&client,&m, 150));}
    t[48] = 104; // change back data
    
    // verify basic auth works
    {let m = Msg::from_bytes(&t[..]);
     assert!(Msg::auth(&client,&m, 150));}

    // compare original and latest
    assert_eq!(pt,t);
}
