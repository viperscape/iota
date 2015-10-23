use ::{Msg,Client};

use std::time::Duration;
use std::net::{SocketAddrV4,
               UdpSocket,
               Ipv4Addr};

pub const MAX_LEN: usize = 1024;

pub fn listen(ip: Ipv4Addr, port: u16) {
    if let Some(mut socket) = UdpSocket::bind(SocketAddrV4::new(ip, port)).ok() {
        socket.set_read_timeout(Some(Duration::new(1,0)));
        
        let client = Client::blank();
        let msg = Msg::new(&client,&b"Hello".to_vec()[..]);
        
        socket.send_to(&msg.into_vec()[..],(ip,port));
        
        let msg = collect_msg(&mut socket);

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
}

pub fn collect_msg(socket: &mut UdpSocket) -> Msg {
    let mut buf = [0; MAX_LEN];
    
    match socket.recv_from(&mut buf) {
        Ok((amt, _src)) => {
            let r = &mut buf[..amt];
            Msg::from_bytes(&r)
        },
        Err(_) => { panic!("unable to collect message") },
    }
}
