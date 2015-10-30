#![allow(unused_imports)]

use clock_ticks::precise_time_ms;
use rand::random;

use byteorder::{ByteOrder, BigEndian};

use ::Msg;

#[derive(Debug)]
pub struct Client {
    // TODO: consider storing tid as [u8;8]
    pub tid: u64, //long term (tombstone) client id, never changes
    et: u64, // epoch time of initial connection
    key: Vec<u8>, //shared key //TODO: consider [u8;8/16]
}

impl Client {
    pub fn blank() -> Client {
        Client {
            tid: 0,
            et: 0,
            key: vec!(),
        }
    }

    pub fn from_msg(msg: &Msg) -> Client {
        Client {
            tid: msg.tid(),
            et: precise_time_ms(),
            key: vec!(),
        }
    }

    pub fn apply_key(&mut self, key: Vec<u8>) {
        if self.key.is_empty() {
            self.key = key;
        }
    }
    
    pub fn key(&self) -> &[u8] {
        &self.key[..]
    }

    pub fn reset_time(&mut self) {
        self.et = precise_time_ms();
    }
}
