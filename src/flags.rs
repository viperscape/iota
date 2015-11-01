#![allow(non_upper_case_globals)]

bitflags! {
    
    flags Flags: u8 {
        const Cmd = 0,
        const Ping = 1, // may be removed in favor of a blank req
        const Req = 1 << 1, // request
        const Resp = 1 << 2, // response
        const Pub = 1 << 3, // publishing to an endpoint
        const G1  = 1 << 4, // guaranteed at least once
        const Bat = 1 << 5, // batching
        const Alt = 1 << 6, // supply alternate encodings
        const Alg = 1 << 7, // alternate hash algorithm
    }
}
