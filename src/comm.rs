use ::{Msg,MsgBuilder,Client,flags};

use std::time::Duration;
use std::net::{SocketAddrV4,
               SocketAddr,
               UdpSocket,
               Ipv4Addr, };


pub const MAX_LEN: usize = 4096;

pub fn listen(ip: Ipv4Addr, port: u16) {
    let src = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        socket.set_read_timeout(Some(Duration::new(1,0)));
        let mut buf = [0; MAX_LEN];
        
        loop {
            let (msg,src) = collect_msg(&mut buf, &mut socket);
            let client = Client::from_msg(&msg);
            
            if Msg::auth(&client,&msg) {
                println!("auth {:?} {:?}",msg.data, msg.flags());
                handler(&client,&msg,src,&mut socket);
            }
            else { println!("not auth") }
        }
        
    }
    else { panic!("cannot bind socket"); }

}

pub fn send_ping(ip: Ipv4Addr, port: u16) {
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
            println!("auth {:?} {:?}",msg.data, msg.flags());
        }
        else { println!("not auth") }
    }
}


pub fn reqres() {
    use std::thread;
    
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 12345;
    let s = thread::spawn(move || { listen(ip,port) });
    send_ping(ip,port);
}

/// command handler for flags
pub fn handler (client: &Client,
                msg: &Msg,
                src: SocketAddr, socket: &mut UdpSocket) {
    let flags = msg.flags();
    let pingreq = flags::Ping|flags::Req;
    match flags {
        &pingreq => { // send a ping reply
            ping_res(client,src,socket);
        },
    }
}

pub fn ping_req(client: &Client,
                src: SocketAddrV4, socket: &mut UdpSocket) {
    let d = [0u8];
    let m = MsgBuilder::new(client,&d[..]).
        flag(flags::Ping).flag(flags::Req).build();
    let r = socket.send_to(&m.into_vec()[..],src);
    println!("ping req: {:?}",r);
}
pub fn ping_res(client: &Client,
                src: SocketAddr, socket: &mut UdpSocket) {
    let d = [0u8];
    let m = MsgBuilder::new(client,&d[..]).
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
