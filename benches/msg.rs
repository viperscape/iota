#![feature(test)]

extern crate test;

extern crate crypto;
extern crate clock_ticks;
extern crate rand;
extern crate byteorder;
#[macro_use] extern crate bitflags;

extern crate iota;


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
use byteorder::{ByteOrder, BigEndian};

use iota::{Msg,MsgBuilder,Client,flags};


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

/*
#[bench]
fn auth(b:&mut Bencher) {
    b.iter(||{auth_ok();});
}

#[bench]
fn auth_alg(b:&mut Bencher) {
    b.iter(||{auth_ok_alg();});
}
*/
