
#![allow(unused_must_use)]

extern crate clock_ticks;
extern crate rand;
extern crate byteorder;

use iota::{Msg,MsgBuilder,Client,flags,MAX_DATA};
use iota::comm::{Handler,Store,collect_msg,manage,ping_req,ping_resp};
use iota::comm;

use self::clock_ticks::precise_time_s;
use self::rand::random;

use self::byteorder::{ByteOrder, BigEndian};

use std::time::Duration;
use std::net::{SocketAddrV4,
               SocketAddr,
               UdpSocket,
               Ipv4Addr, };
use std::io;

const SRC_PORT: u16 = 55265;

pub fn listen<H:Handler>(ip: Ipv4Addr, port: u16, handler:&mut H, store: &mut Store) {
    
    let src = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        socket.set_read_timeout(Some(Duration::new(2,0)));
        let mut buf = [0; MAX_DATA];
        
        loop {
            if let Some((msg,src)) = collect_msg(&mut buf, &mut socket).ok() {
                let client = Client::blank();
                
                if Msg::auth(&client,&msg, 150) {
                    println!("dest auth {:?} {:?}",msg.data, msg.flags());
                    manage(&client,&msg,src,&mut socket, handler,store);
                }
                else { println!("dest not auth") }
            }
            else { break }
        }
        
    }
    else { panic!("cannot bind socket"); }

}

pub fn send_ping<H:Handler>(ip: Ipv4Addr, port: u16,handler:&mut H, store: &mut Store) -> Result<(),io::Error> {
    let src = SocketAddrV4::new(ip, SRC_PORT);
    let dest = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        let client = Client::blank();
        
        let m = ping_req(&client);
        let r = socket.send_to(&m[..],dest);
        println!("ping req: {:?}",r);
        
        socket.set_read_timeout(Some(Duration::new(1,0)));
        let mut buf = [0; MAX_DATA];

        if let Some((msg,src)) = collect_msg(&mut buf, &mut socket).ok() {
            let client = Client::from_msg(&msg);
            if Msg::auth(&client,&msg, 150) {
                println!("src ping res auth {:?} {:?}",msg.data, msg.flags());
                manage(&client,&msg,src,&mut socket,handler,store);
            }
            else { println!("src not auth") }
        }
    }

    Ok(())
}

pub fn send_req<H:Handler>(ip: Ipv4Addr, port: u16,handler:&mut H, store: &mut Store) {
    let src = SocketAddrV4::new(ip, SRC_PORT);
    let dest = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        let client = Client::blank();

        let d = [random::<u8>()];
        let m = MsgBuilder::new(&client,&d[..]).
            flag(flags::Req).route(53).build();
        let r = socket.send_to(&m.into_vec()[..],dest);
        println!("src send req {:?}",r);
        
        socket.set_read_timeout(Some(Duration::new(2,0)));
        let mut buf = [0; MAX_DATA];

        if let Some((msg,src)) = collect_msg(&mut buf, &mut socket).ok() {
            let client = Client::from_msg(&msg);
            if Msg::auth(&client,&msg, 150) {
                println!("src res auth {:?} {:?}",msg.data, msg.flags());
                manage(&client,&msg,src,&mut socket,handler,store);
            }
            else { println!("src not auth") }
        }
    }
}

pub fn send_pub<H:Handler>(ip: Ipv4Addr, port: u16, _handler:&mut H, store: &mut Store) {
    let src = SocketAddrV4::new(ip, SRC_PORT);
    let dest = SocketAddrV4::new(ip, port);
    if let Some(socket) = UdpSocket::bind(src).ok() {
        let client = Client::blank();

        let d = [1];
        let m = MsgBuilder::new(&client,&d[..]).
            flag(flags::Pub).route(53).build();
        let r = socket.send_to(&m.into_vec()[..],dest);
        println!("send pub {:?}",r);
    }
}

pub fn send_sess<H:Handler>(ip: Ipv4Addr, port: u16, handler:&mut H, store: &mut Store) {
    let src = SocketAddrV4::new(ip, SRC_PORT);
    let dest = SocketAddrV4::new(ip, port);
    if let Some(mut socket) = UdpSocket::bind(src).ok() {
        let mut client = Client::blank();

        let m = comm::enc_sess(&mut client);
        handler.set_session(client.tid,client.session().clone());

        let r = socket.send_to(&m[..],dest);
        println!("send sess {:?}",r);

        socket.set_read_timeout(Some(Duration::new(2,0)));
        let mut buf = [0; MAX_DATA];

        if let Some((msg,src)) = collect_msg(&mut buf, &mut socket).ok() {
            let client = Client::from_msg(&msg);
            if Msg::auth(&client,&msg, 150) {
                println!("src res auth {:?} {:?}",msg.data, msg.flags());
                manage(&client,&msg,src,&mut socket,handler,store);
            }
            else { println!("src not auth") }
        }
    }
}


// example req res
pub fn reqres<H:Handler+Send+'static+Clone>(handler:H) {
    use std::thread;
    let mut handler = handler.clone();
    let mut handler2 = handler.clone();
    
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 12345;

    let mut store = Store::new();
    let mut store_dest = Store::new();

    thread::spawn(move || {
        send_ping(ip,port,&mut handler, &mut store);
        send_sess(ip,port,&mut handler, &mut store);

        send_ping(ip,port,&mut handler, &mut store);

        send_req(ip,port,&mut handler, &mut store);
        
        send_pub(ip,port,&mut handler, &mut store);
        send_req(ip,port,&mut handler, &mut store);
    });
    
    listen(ip,port,&mut handler2, &mut store_dest);
}
