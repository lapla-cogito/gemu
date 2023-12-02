#[derive(Clone, Debug, Default)]
pub struct Interrupts {
    pub ime: bool,
    pub i_enable: u8,
    pub i_flag: u8,
}

impl Interrupts {
    pub fn irq(&mut self, val: u8) {
        self.i_flag |= val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF0F => self.intr_flags,
            0xFFFF => self.intr_enable,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF0F => self.intr_flags = val,
            0xFFFF => self.intr_enable = val,
            _ => unreachable!(),
        }
    }

    pub fn get_int(&self) -> u8 {
        self.i_flag & self.i_enable & 0b11111
    }
}
