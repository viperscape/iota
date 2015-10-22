extern crate crypto;

pub struct Client {
    tid: u64, //long term (tombstone) client id, never changes
    t: f64, // time of initial connection
    
}


pub struct Msg {
    mid: u32, //short term, message id, always changes
    dt: f64, // delta time since initial connection

    data: [u8],
}
