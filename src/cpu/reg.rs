#[derive(Clone, Copy, Debug, Default)]
pub struct Registers {
    pub pc: u16, // program counter
    pub sp: u16, // stack pointer
    pub a: u8,   // accumulator
    pub f: u8,   // flags
    pub b: u8,   // general purpose registers
    pub c: u8,   // general purpose registers
    pub d: u8,   // general purpose registers
    pub e: u8,   // general purpose registers
    pub h: u8,   // general purpose registers
    pub l: u8,   // general purpose registers
}

impl Registers {
    // 16-bit registers
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    // 16-bit registers (mut)
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xf0) as u8; // The lower 4 bits of F register are always 0
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn zf(&self) -> bool {
        (self.f & 0b10000000) > 0
    }

    pub fn cf(&self) -> bool {
        (self.f & 0b00010000) > 0
    }

    pub fn set_zf(&mut self, val: bool) {
        if val {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
    }

    pub fn set_nf(&mut self, val: bool) {
        if val {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
    }

    pub fn set_hf(&mut self, val: bool) {
        if val {
            self.f |= 0b00100000;
        } else {
            self.f &= 0b11011111;
        }
    }

    pub fn set_cf(&mut self, val: bool) {
        if val {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
    }
}
