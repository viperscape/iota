#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_imports)]

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

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::{Mac};

pub const MAX_LEN: usize = 1400;


/// command handler for flags
pub fn manage<H:Handler>
    (client: &Client,
     msg: &Msg,
     dest: SocketAddr,
     socket: &mut UdpSocket,
     handler: &mut H) {
        let (flags,rt) = msg.flags();
        let has_sess = handler.get_session(client.tid).is_some();
        
        if flags.is_empty() {
            if rt == 0 { // session start
                let sess = dec_sess(&client,&msg);

                handler.set_session(client.tid,sess);
                
                let m = MsgBuilder::new(client,&msg.data[..]).
                    route(1).build();
                let r = socket.send_to(&m.into_vec()[..],dest);
                println!("send sess resp {:?}",r);
            }
            if rt == 1 { // session response, now negotiated
                if let Some(sess_req) = handler.get_session(client.tid) {
                    let sess_resp = dec_sess(&client,&msg);

                    if *sess_req != sess_resp { println!("session invalid!"); }
                    else { println!("session valid"); }
                }
                else { println!("sess not set!"); }
            }
        }
        else if has_sess {
            if flags.contains(flags::Ping|flags::Req) { // send a ping reply
                let m = ping_res(client,msg.data);
                let r = socket.send_to(&m[..],dest);
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
        else { println!("handler requires session") }
    }

/// encrypt a session id as a new Msg
pub fn enc_sess(client: &mut Client) -> Vec<u8> {
    let t = precise_time_ms();

    let mut esess = [0u8;16];
    {
        let mut enc;
        
        {let key = client.key();
         enc = aessafe::AesSafe128Encryptor::new(&key[..]);}

        enc.encrypt_block(client.session(), &mut esess);
    }
    
    let mut m = MsgBuilder::new(client, &esess[..]).build();
    m.into_vec()
}

/// decrypt a session from msg data
pub fn dec_sess(client: &Client, msg: &Msg) -> [u8;16] {
    let t = msg.time();
    let key = client.key();
    let mut sess = [0u8;16];
    
    let mut dec = aessafe::AesSafe128Decryptor::new(&key[..]);
    dec.decrypt_block(&msg.data, &mut sess);

    sess
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

use std::io;
pub fn collect_msg<'d> (buf: &'d mut [u8;MAX_LEN], socket: &mut UdpSocket) -> Result<(Msg<'d>,SocketAddr),io::Error> {
    match socket.recv_from(buf) {
        Ok((amt, src)) => {
            let r = &mut buf[..amt];
            Ok((Msg::from_bytes(r),src))
        },
        Err(e) => { Err(e) },
    }
}

pub trait Handler {
    fn ping(&mut self, dt: f32);
    fn publish(&mut self, tid: u64, rt: u16, data: &[u8]);
    fn request(&mut self, rt: u16, buf: &mut [u8]) -> usize;
    fn set_session(&mut self, tid: u64, sess: [u8;16]);
    fn get_session(&mut self, tid: u64) -> Option<&[u8;16]>;
    
    //fn list(&self);
    //fn batch(&mut self, tid: u64, n: u8, data: &[u8]);
    //fn new_batch(&mut self, tid: u64, rt: u8);
}
