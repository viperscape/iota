#![allow(unused_must_use)]
#![allow(unused_variables)]

use ::{Msg,MsgBuilder,Client,flags};

use clock_ticks::precise_time_s;
use rand::random;

use byteorder::{ByteOrder, BigEndian};

use std::time::Duration;
use std::net::{SocketAddrV4,
               SocketAddr,
               UdpSocket,
               Ipv4Addr, };


pub const MAX_LEN: usize = 1400;


/// command handler for flags
pub fn manage<H:Handler>
    (client: &Client,
     msg: &Msg,
     dest: SocketAddr,
     socket: &mut UdpSocket,
     handler: &mut H) {
        let (flags,rt) = msg.flags();
        
        if flags.contains(flags::Ping|flags::Req) { // send a ping reply
            let m = ping_res(client,msg.data);
            let r = socket.send_to(&m[..],dest);
            println!("ping res: {:?}",r);
            if flags.contains(flags::G1) { println!("guarantee unimpl"); }
        }
        else if flags.contains(flags::Ping|flags::Res) {
            let d = BigEndian::read_f32(msg.data);
            handler.ping(precise_time_s() as f32 - d);
        }
        else if flags == flags::Req { //FIXME: should probably use intersect
            let mut buf = [0u8;MAX_LEN];
            let amt = handler.request(rt,&mut buf);
            println!("req buf {:?}",buf[0]);
            
            let m = MsgBuilder::new(client,&buf[..amt]).
                flag(flags::Res).route(rt).build();
            let r = socket.send_to(&m.into_vec()[..],dest);
            println!("send res {:?}",r);
        }
        else if flags == flags::Pub {
            handler.publish(client.tid,rt,msg.data);
        }
    }

/// build ping request message
pub fn ping_req(client: &Client,) -> Vec<u8> {
    let mut d = &mut [0;4];
    BigEndian::write_f32(d, precise_time_s() as f32);
    let m = MsgBuilder::new(client,&d[..]).
        flag(flags::Ping).flag(flags::Req).build();

    m.into_vec()
}

/// build ping response message
pub fn ping_res(client: &Client,
                data: &[u8],) -> Vec<u8> {
    let m = MsgBuilder::new(client,&data[..]).
        flag(flags::Ping).flag(flags::Res).build();
    m.into_vec()
}

pub fn collect_msg<'d> (buf: &'d mut [u8;MAX_LEN], socket: &mut UdpSocket) -> (Msg<'d>,SocketAddr) {
    match socket.recv_from(buf) {
        Ok((amt, src)) => {
            let r = &mut buf[..amt];
            (Msg::from_bytes(r),src)
        },
        Err(e) => { panic!("unable to collect message, {:?}",e) },
    }
}


pub trait Handler {
    fn ping(&mut self, dt: f32);
    fn publish(&mut self, tid: u64, rt: u16, data: &[u8]);
    fn request(&mut self, rt: u16, buf: &mut [u8]) -> usize;
    fn list(&self);
    //fn batch(&mut self, tid: u64, n: u8, data: &[u8]);
    //fn new_batch(&mut self, tid: u64, rt: u8);
}
