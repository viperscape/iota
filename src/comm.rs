use ::{Msg,MsgBuilder,Client,flags};

use clock_ticks::precise_time_s;

use byteorder::{ByteOrder, BigEndian};

use std::time::Duration;
use std::net::{SocketAddrV4,
               SocketAddr,
               UdpSocket,
               Ipv4Addr, };


pub const MAX_LEN: usize = 4096;

pub fn listen<H:Handler>(ip: Ipv4Addr, port: u16, handler:&mut H) {
    
    let src = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        socket.set_read_timeout(Some(Duration::new(5,0)));
        let mut buf = [0; MAX_LEN];
        
        loop {
            let (msg,src) = collect_msg(&mut buf, &mut socket);
            let client = Client::from_msg(&msg);
            
            if Msg::auth(&client,&msg) {
                println!("main listen auth {:?} {:?}",msg.data, msg.flags());
                manage(&client,&msg,src,&mut socket, handler);
            }
            else { println!("not auth") }
        }
        
    }
    else { panic!("cannot bind socket"); }

}

pub fn send_ping<H:Handler>(ip: Ipv4Addr, port: u16,handler:&mut H) {
    let src = SocketAddrV4::new(ip, 55265);
    let dest = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        let client = Client::blank();
        ping_req(&client,dest,&mut socket);

        socket.set_read_timeout(Some(Duration::new(1,0)));
        let mut buf = [0; MAX_LEN];
        let (msg,src) = collect_msg(&mut buf, &mut socket);
        let client = Client::from_msg(&msg);
        if Msg::auth(&client,&msg) {
            println!("ping res auth {:?} {:?}",msg.data, msg.flags());
            manage(&client,&msg,src,&mut socket,handler);
        }
        else { println!("not auth") }
    }
}

pub fn send_req<H:Handler>(ip: Ipv4Addr, port: u16,handler:&mut H) {
    let src = SocketAddrV4::new(ip, 55265);
    let dest = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        let client = Client::blank();

        let d = [0];
        let m = MsgBuilder::new(&client,&d[..]).
            flag(flags::Req).route(53).build();
        let r = socket.send_to(&m.into_vec()[..],dest);
        println!("send req {:?}",r);
        
        socket.set_read_timeout(Some(Duration::new(2,0)));
        let mut buf = [0; MAX_LEN];
        let (msg,src) = collect_msg(&mut buf, &mut socket);
        let client = Client::from_msg(&msg);
        if Msg::auth(&client,&msg) {
            println!("res auth {:?} {:?}",msg.data, msg.flags());
            manage(&client,&msg,src,&mut socket,handler);
        }
        else { println!("not auth") }
    }
}

// example req res
pub fn reqres<H:Handler+Send+'static+Clone>(handler:H) {
    use std::thread;
    let mut handler = handler.clone();
    let mut handler2 = handler.clone();
    
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 12345;
    let s = thread::spawn(move || { listen(ip,port,&mut handler) });
    send_ping(ip,port,&mut handler2);
    send_req(ip,port,&mut handler2);
}

/// command handler for flags
pub fn manage<H:Handler>
    (client: &Client,
     msg: &Msg,
     src: SocketAddr,
     socket: &mut UdpSocket,
     handler: &mut H) {
        let (flags,rt) = msg.flags();

        if flags.contains(flags::Ping|flags::Req) { // send a ping reply
            ping_res(client,msg.data,src,socket);
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
            let r = socket.send_to(&m.into_vec()[..],src);
            println!("send res {:?}",r);
        }
        else if flags == flags::Pub {
            handler.publish(client.tid,rt,msg.data);
        }
    }

pub fn ping_req(client: &Client,
                src: SocketAddrV4,
                socket: &mut UdpSocket) {
    let mut d = &mut [0u8;4];
    BigEndian::write_f32(d, precise_time_s() as f32);
    
    let m = MsgBuilder::new(client,&d[..]).
        flag(flags::Ping).flag(flags::Req).build();
    let r = socket.send_to(&m.into_vec()[..],src);
    
    println!("ping req: {:?}",r);
}
pub fn ping_res(client: &Client,
                data: &[u8],
                src: SocketAddr,
                socket: &mut UdpSocket) {
    let m = MsgBuilder::new(client,&data[..]).
        flag(flags::Ping).flag(flags::Res).build();
    let r = socket.send_to(&m.into_vec()[..],src);
    
    println!("ping res: {:?}",r);
}

pub fn collect_msg<'d> (buf: &'d mut [u8;MAX_LEN], socket: &mut UdpSocket) -> (Msg<'d>,SocketAddr) {
    match socket.recv_from(buf) {
        Ok((amt, src)) => {
            let r = &mut buf[..amt];
            (Msg::from_bytes(r),src)
        },
        Err(_) => { panic!("unable to collect message") },
    }
}


pub trait Handler {
    fn ping(&mut self, dt: f32);
    fn publish(&mut self, tid: u64, rt: u8, data: &[u8]);
    fn request(&mut self, rt: u8, buf: &mut [u8]) -> usize;
    fn list(&self);
}
