extern crate iota;
use iota::{Msg,Client};

use std::time::Duration;
use std::net::{SocketAddrV4, TcpStream, UdpSocket, TcpListener, Ipv4Addr};

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 12345;
    
    if let Some(mut socket) = UdpSocket::bind(SocketAddrV4::new(ip, port)).ok() {
        socket.set_read_timeout(Some(Duration::new(1,0)));
        
        let client = Client::blank();
        let msg = Msg::new(&client,&b"Hello".to_vec()[..]);
        
        socket.send_to(&msg.into_vec()[..],(ip,port));
        
        let msg = collect_msg(&client,&mut socket);

        if Msg::auth(&client,&msg) {
            println!("auth {:?}",msg.data);
        }
        else { println!("not auth {:?}",msg.mid) }

    }

}

fn collect_msg(client: &Client, socket: &mut UdpSocket) -> Msg {
    let mut buf = [0; 1024];
    
    match socket.recv_from(&mut buf) {
        Ok((amt, src)) => {
            let r = &mut buf[..amt];
            Msg::from_bytes(&client,&r)
        },
        Err(_) => {panic!("whoa")},
    }
}
