#[derive(Eq,PartialEq)]
pub enum Flags {
    Ping = 1,
    Req = 1 << 1,
    Res = 1 << 2,
    Pub = 1 << 3,
    Cmd = 1 << 4,
    
    R1 = 1 << 5,
    R2 = 1 << 6,
    R3 = 1 << 7,
}
use std::ops::BitOr;
impl BitOr for Flags {
    type Output = u8;
    fn bitor(self, _rhs: Flags) -> u8 {
        self as u8 | _rhs as u8
    }
}
