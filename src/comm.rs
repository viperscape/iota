use ::{Msg,MsgBuilder,Client,Flags};

use std::time::Duration;
use std::net::{SocketAddrV4,
               UdpSocket,
               Ipv4Addr};


pub const MAX_LEN: usize = 4096;

pub fn listen(ip: Ipv4Addr, port: u16) {
    if let Some(mut socket) = UdpSocket::bind(SocketAddrV4::new(ip, port)).ok() {
        socket.set_read_timeout(Some(Duration::new(1,0)));
        
        let client = Client::blank();
        let data = &b"Hello"[..];
        let msg = MsgBuilder::new(&client,data).build();
        
        socket.send_to(&msg.into_vec()[..],(ip,port));

        let mut buf = [0; MAX_LEN];
        let msg = collect_msg(&mut buf, &mut socket);

        let client = Client::from_msg(&msg);
        
        if Msg::auth(&client,&msg) {
            println!("auth {:?}",msg.data);
        }
        else { println!("not auth") }

    }
    else { panic!("cannot bind socket"); }

}

pub fn reqres() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 12345;
    listen(ip,port);
}

/// command handler for flags
pub fn handler (msg: &Msg) {
    let flags = msg.flags();
    match flags {
        Ping => {},
       // _ => {},
    }
}

pub fn collect_msg<'d> (buf: &'d mut [u8;MAX_LEN], socket: &mut UdpSocket) -> Msg<'d> {
    match socket.recv_from(buf) {
        Ok((amt, _src)) => {
            let r = &mut buf[..amt];
            Msg::from_bytes(r)
        },
        Err(_) => { panic!("unable to collect message") },
    }
}
