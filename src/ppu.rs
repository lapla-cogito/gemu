use crate::constants::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OAMScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    mode: Mode,
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    vram: Box<[u8; 0x2000]>,
    oam: Box<[u8; 0xa0]>,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: Mode::OAMScan,
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0x00,
            obp0: 0x00,
            obp1: 0x00,
            wy: 0,
            wx: 0,
            vram: Box::new([0; 0x2000]),
            oam: Box::new([0; 0xa0]),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => {
                if self.mode == Mode::Drawing {
                    0xff
                } else {
                    self.vram[addr as usize & 0x1fff]
                }
            }
            0xfe00..=0xfe9f => {
                if self.mode == Mode::OAMScan || self.mode == Mode::Drawing {
                    0xff
                } else {
                    self.oam[addr as usize & 0x9f]
                }
            }
            0xff40 => self.lcdc,
            0xff41 => 0x80 | self.stat | self.mode as u8,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0xFF,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x8000..=0x9FFF => {
                if self.mode != Mode::Drawing {
                    self.vram[addr as usize & 0x1fff] = data;
                }
            }
            0xFE00..=0xFE9F => {
                if self.mode != Mode::Drawing && self.mode != Mode::OAMScan {
                    self.oam[addr as usize & 0xff] = data;
                }
            }
            0xFF40 => self.lcdc = data,
            0xFF41 => self.stat = (self.stat & LYC_EQ_LY) | (data & 0xF8),
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF44 => {}
            0xFF45 => self.lyc = data,
            0xFF47 => self.bgp = data,
            0xFF48 => self.obp0 = data,
            0xFF49 => self.obp1 = data,
            0xFF4A => self.wy = data,
            0xFF4B => self.wx = data,
            _ => unreachable!(),
        }
    }
}
