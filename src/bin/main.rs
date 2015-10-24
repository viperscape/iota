extern crate iota;
use iota::{Msg,Client,comm};

use std::time::Duration;
use std::net::{SocketAddrV4,
               UdpSocket,
               Ipv4Addr};

fn main() {
    comm::reqres();
}
