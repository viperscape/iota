#![allow(non_upper_case_globals)]

bitflags! {
    
    flags Flags: u8 {
        const Ping = 1,
        const Req = 1 << 1,
        const Res = 1 << 2,
        const Pub = 1 << 3,
        const Cmd = 1 << 4,
        
        const R1 = 1 << 5,
        const R2 = 1 << 6,
        const R3 = 1 << 7,
    }
}
