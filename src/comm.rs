#![allow(unused_must_use)]
#![allow(unused_variables)]

use ::{Msg,MsgBuilder,Client,flags};

use clock_ticks::precise_time_s;
use clock_ticks::precise_time_ms;
use rand::random;

use byteorder::{ByteOrder, BigEndian};

use std::time::Duration;
use std::net::{SocketAddrV4,
               SocketAddr,
               UdpSocket,
               Ipv4Addr, };

use crypto::aessafe;
use crypto::aes;
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor};

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
        else if flags == flags::Cmd {
            if rt == 0 { // session start
                let sess = decrypt_sess(&client,&msg);
                println!("session: {:?}",sess);
            }
        }
    }

/// encrypt a session id as a new Msg
pub fn enc_sess(client: &mut Client) -> Vec<u8> {
    let t = precise_time_ms();
    let mut bt = &mut [0u8;4];
    BigEndian::write_u32(bt,t as u32);

    let sess = client.reset_session();
    let key = //client.key();
        vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
             0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    
    let mut esess = [0u8;16];
    let mut tsess = &mut [0u8;16];
    BigEndian::write_u32(tsess,sess);
    
    let mut enc = aessafe::AesSafe128Encryptor::new(&key);
    enc.encrypt_block(&tsess[..], &mut esess);

    let mut tmp = [0u8;16];
    let mut dec = aessafe::AesSafe128Decryptor::new(&key);
    dec.decrypt_block(&esess, &mut tmp);

    let nsess = BigEndian::read_u32(&tmp[..]);

    assert_eq!(nsess,sess);
    
    let mut m = MsgBuilder::new(client, &esess[..]);
    // reset time in msg to match
    {let mut mpt = &mut m.0[44..48];
     BigEndian::write_u32(mpt,t as u32);}
    
    m.build().into_vec()
}

/// decrypt a session from msg data
pub fn decrypt_sess(client: &Client, msg: &Msg) -> Option<u32> {
    let t = msg.time();
    let key = client.key();
    let mut sess = [0u8;32];

    None
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
