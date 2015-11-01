#![allow(unused_imports)]

extern crate iota;
use iota::comm::{Handler,};

mod net;

use iota::{Msg,Client,comm};

use std::time::Duration;
use std::net::{SocketAddrV4,
               UdpSocket,
               Ipv4Addr};
use std::collections::HashMap;
    
fn main() {
    net::reqres(Worker {
        state: HashMap::new(),
        sess: HashMap::new(),
    });
}

#[derive(Clone)]
struct Worker {
    state: HashMap<u16, (u64,bool)>, // route, state
    sess: HashMap<u64, [u8;16]>, //tid, session
}
impl Handler for Worker {
    fn ping (&mut self, dt: f32) {
        println!("dt: {:?}",dt);
    }
    fn publish(&mut self, tid: u64, rt: u16, data: &[u8]) {
        self.state.insert(rt, (tid,data[0] == 1));
    }
    fn request(&mut self, rt: u16, buf: &mut [u8]) -> usize {
        if let Some(ref n) = self.state.get(&rt) {
            buf[0] = n.1 as u8;
            1
        }
        else { buf[0] = 0;
               1 }
    }
    
    fn set_session(&mut self, tid: u64, sess: [u8;16]) {
        self.sess.insert(tid,sess);
    }

    fn get_session(&mut self, tid: u64) -> Option<&[u8;16]> {
        self.sess.get(&tid)
    }
}
