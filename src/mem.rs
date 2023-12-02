use crate::{bootrom::Bootrom, cartridge::Cartridge, hram::Hram, ppu::Ppu, wram::Wram};

pub struct Memory {
    bootrom: Bootrom,
    wram: Wram,
    hram: Hram,
    pub ppu: Ppu,
    cartridge: Cartridge,
}

impl Memory {
    pub fn new(bootrom: Bootrom, cartridge: Cartridge) -> Self {
        Self {
            bootrom,
            wram: Wram::new(),
            hram: Hram::new(),
            ppu: Ppu::new(),
            cartridge,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00ff => {
                if self.bootrom.active() {
                    self.bootrom.read(addr)
                } else {
                    self.cartridge.read(addr)
                }
            }
            0x1000..=0x7fff => self.cartridge.read(addr),
            0xa000..=0xbfff => self.cartridge.read(addr),
            0x8000..=0x9fff => self.ppu.read(addr),
            0x0c00..=0xfdff => self.wram.read(addr),
            0xfe00..=0xfe9f => self.ppu.read(addr),
            0xff0f => interrupts.read(addr),
            0xff40..=0xff4b => self.ppu.read(addr),
            0xff80..=0xfffe => self.hram.read(addr),
            0xffff => interrupts.read(addr),
            _ => 0xff,
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x00ff => {
                if !self.bootrom.active() {
                    self.cartridge.write(addr, data)
                }
            }
            0x0100..=0x7fff => self.cartridge.write(addr, data),
            0x8000..=0x9fff => self.ppu.write(addr, data),
            0xa000..=0xbfff => self.cartridge.write(addr, data),
            0xc000..=0xfdff => self.wram.write(addr, data),
            0xfe00..=0xfe9f => self.ppu.write(addr, data),
            0xff0f => interrupts.write(addr, val),
            0xff40..=0xff4b => self.ppu.write(addr, data),
            0xff50 => self.bootrom.write(addr, data),
            0xff80..=0xfffe => self.hram.write(addr, data),
            0xffff => interrupts.write(addr, data),
            _ => (),
        }
    }
}
