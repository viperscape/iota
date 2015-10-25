#![allow(non_upper_case_globals)]

bitflags! {
    
    flags Flags: u8 {
        // const None = 0, TODO: no flags needs a flag! and purpose
        const Ping = 1, // may be removed in favor of a blank req
        const Req = 1 << 1, // request
        const Res = 1 << 2, // response
        const Pub = 1 << 3, // publishing to an endpoint
        const G1  = 1 << 4, // guaranteed at least once

        // currently reserved bits for future extension
        const R1 = 1 << 5,
        const R2 = 1 << 6,
        const R3 = 1 << 7,
    }
}
