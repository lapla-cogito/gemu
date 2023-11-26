use crate::{bootrom::Bootrom, hram::Hram, ppu::Ppu, wram::Wram};

pub struct Memory {
    bootrom: Bootrom,
    wram: Wram,
    hram: Hram,
    ppu: Ppu,
}

impl Memory {
    pub fn new(bootrom: Bootrom) -> Self {
        Self {
            bootrom,
            wram: Wram::new(),
            hram: Hram::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00ff => {
                if self.bootrom.active() {
                    self.bootrom.read(addr)
                } else {
                    0xff
                }
            }
            0x0c00..=0xfdff => self.wram.read(addr),
            0xff80..=0xfffe => self.hram.read(addr),
            0x8000..=0x9fff => self.ppu.read(addr),
            0xfe00..=0xfe9f => self.ppu.read(addr),
            0xff40..=0xff4b => self.ppu.read(addr),
            _ => panic!("Invalid read address: {:#06x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0xc000..=0xfdff => self.wram.write(addr, data),
            0xff50 => self.bootrom.write(addr, data),
            0xff80..=0xfffe => self.hram.write(addr, data),
            0x8000..=0x9fff => self.ppu.write(addr, data),
            0xfe00..=0xfe9f => self.ppu.write(addr, data),
            0xff40..=0xff4b => self.ppu.write(addr, data),
            _ => panic!("Invalid write address: {:#06x}", addr),
        }
    }
}
