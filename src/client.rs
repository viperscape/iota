#![allow(unused_imports)]

use clock_ticks::precise_time_ms;
use rand::random;

use byteorder::{ByteOrder, BigEndian};

use ::Msg;

#[derive(Debug)]
pub struct Client {
    pub tid: u64, //long term (tombstone) client id, never changes
    et: u64, // epoch time of initial connection
    key: [u8;16], //shared key
    sess: [u8;16], //session id
}

impl Client {
    pub fn blank() -> Client {
        Client {
            tid: 0,
            et: 0,
            key: [0u8;16],
            sess: [0u8;16],
        }
    }

    pub fn from_msg(msg: &Msg) -> Client {
        Client {
            tid: msg.tid(),
            et: precise_time_ms(),
            key: [0u8;16],
            sess: [0u8;16],
        }
    }

    pub fn apply_key(&mut self, key: [u8;16]) {
        self.key = key;
    }

    pub fn apply_sess(&mut self, sess: [u8;16]) {
        self.sess = sess;
    }
    
    pub fn key(&self) -> &[u8;16] {
        &self.key
    }

    pub fn session(&self) -> &[u8;16] {
        &self.sess
    }

    pub fn reset_time(&mut self) {
        self.et = precise_time_ms();
    }

    pub fn reset_session(&mut self) -> &[u8;16] {
        self.sess = [random::<u8>();16];

        &self.sess
    }
}
