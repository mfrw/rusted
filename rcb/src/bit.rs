use bitflags::bitflags;

use std::fmt;

bitflags! {
    struct Flags: u32 {
        const FLAG_A = 0b00000001;
        const FLAG_B = 0b00000010;
        const FLAG_C       = 0b00000100;
        const FLAG_ABC     = Self::FLAG_A.bits
                            | Self::FLAG_B.bits
                            | Self::FLAG_C.bits;
    }
}

impl Flags {
    pub fn clear(&mut self) -> &mut Self {
        self.bits = 0;
        self
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits)
    }
}

pub fn run() {
    let e1 = Flags::FLAG_A | Flags::FLAG_C;
    let e2 = Flags::FLAG_B | Flags::FLAG_C;
    assert_eq!((e1 | e2), Flags::FLAG_ABC);
    assert_eq!((e1 & e2), Flags::FLAG_C);
    assert_eq!((e1 - e2), Flags::FLAG_A);
    assert_eq!(!e2, Flags::FLAG_A);

    let mut flags = Flags::FLAG_ABC;
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(
        format!("{}", flags.clear()),
        "00000000000000000000000000000000"
    );
    assert_eq!(format!("{:?}", Flags::FLAG_B), "FLAG_B");
    assert_eq!(
        format!("{:?}", Flags::FLAG_A | Flags::FLAG_B),
        "FLAG_A | FLAG_B"
    );
}
