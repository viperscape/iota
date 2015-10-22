//extern crate crypto;
//use crypto::digest::Digest;
//use crypto::sha2::Sha256;


fn main() {
    use std::time::Duration;
    use std::net::{SocketAddrV4, TcpStream, UdpSocket, TcpListener, Ipv4Addr};
    
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 12345;
    
    if let Some(mut socket) = UdpSocket::bind(SocketAddrV4::new(ip, port)).ok() {
        socket.set_read_timeout(Some(Duration::new(1,0)));
        let mut v = vec!();
        for n in "hello".bytes() { v.push(n); }
        
        socket.send_to(&v,(ip,port));
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let buf = &mut buf[..amt];
                println!("{:?}",(buf,src));
            },
            Err(_) => {panic!("whoa")},
        }

    }
}
