#![feature(test)]

extern crate test;
extern crate iota;

use iota::{Client};
use iota::comm;

#[test]
fn enc() {
    let mut client = Client::blank();
    comm::enc_sess(&mut client);
}
