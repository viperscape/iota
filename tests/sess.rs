#![feature(test)]

extern crate test;
extern crate iota;
extern crate rand;

use rand::random;

use iota::{Client,Msg};
use iota::comm;

#[test]
fn enc_dec() {
    let mut client = Client::blank();
    let m = comm::enc_sess(&mut client);
    let m = Msg::from_bytes(&m);
    let dsess = comm::dec_sess(&mut client,&m);

    assert_eq!(&client.session()[..],&dsess[..]);
}
