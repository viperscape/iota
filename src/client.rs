#![allow(unused_imports)]

use clock_ticks::precise_time_ms;
use rand::random;

use byteorder::{ByteOrder, BigEndian};

use ::Msg;

pub const SESS_SIZE: usize = 16;
pub const KEY_SIZE: usize = 16;

#[derive(Debug)]
pub struct Client {
    pub tid: u64, //long term (tombstone) client id, never changes
    et: u64, // epoch time of initial connection
    key: Vec<u8>, //shared key
    sess: Vec<u8>, //session id
}

impl Client {
    pub fn blank() -> Client {
        Client {
            tid: 0,
            et: 0,
            key: Vec::with_capacity(SESS_SIZE),
            sess: Vec::with_capacity(KEY_SIZE),
        }
    }

    pub fn from_msg(msg: &Msg) -> Client {
        Client {
            tid: msg.tid(),
            et: precise_time_ms(),
            key: Vec::with_capacity(SESS_SIZE),
            sess: Vec::with_capacity(KEY_SIZE),
        }
    }

    pub fn apply_key(&mut self, key: Vec<u8>) {
        self.key = key;
    }
    
    pub fn key(&self) -> &[u8] {
        &self.key[..]
    }

    pub fn session(&self) -> &[u8] {
        &self.sess[..]
    }

    pub fn reset_time(&mut self) {
        self.et = precise_time_ms();
    }

    pub fn reset_session(&mut self) -> &[u8] {
        self.sess = [random::<u8>();SESS_SIZE].to_vec();

        &self.sess[..]
    }
}
