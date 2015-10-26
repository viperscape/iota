extern crate iota;
use iota::{Msg,Client,comm};

use std::time::Duration;
use std::net::{SocketAddrV4,
               UdpSocket,
               Ipv4Addr};

fn main() {
    comm::reqres(Worker);
}

struct Worker;
impl comm::Handler for Worker {
    fn ping (&self, dt: f32) {
        println!("dt: {:?}",dt);
    }
}
