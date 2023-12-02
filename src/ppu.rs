use crate::constants::*;
use std::iter;

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
    cycles: u8,
    buffer: Box<[u8; LCD_PIXELS * 4]>,
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
            cycles: 20,
            buffer: Box::new([0; LCD_PIXELS * 4]),
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
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff46 => 0xFF,
            0xff47 => self.bgp,
            0xff48 => self.obp0,
            0xff49 => self.obp1,
            0xff4a => self.wy,
            0xff4b => self.wx,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x8000..=0x9fff => {
                if self.mode != Mode::Drawing {
                    self.vram[addr as usize & 0x1fff] = data;
                }
            }
            0xfe00..=0xfe9f => {
                if self.mode != Mode::Drawing && self.mode != Mode::OAMScan {
                    self.oam[addr as usize & 0xff] = data;
                }
            }
            0xff40 => self.lcdc = data,
            0xff41 => self.stat = (self.stat & LYC_EQ_LY) | (data & 0xF8),
            0xff42 => self.scy = data,
            0xff43 => self.scx = data,
            0xff44 => {}
            0xff45 => self.lyc = data,
            0xff47 => self.bgp = data,
            0xff48 => self.obp0 = data,
            0xff49 => self.obp1 = data,
            0xff4a => self.wy = data,
            0xff4b => self.wx = data,
            _ => unreachable!(),
        }
    }

    fn get_pixel_from_tile(&self, tile_ind: usize, y: u8, x: u8) -> u8 {
        let r = (y * 2) as usize;
        let c = (7 - x) as usize;
        let tile_addr = tile_ind << 4;
        let low = self.vram[(tile_addr | r) & 0x1fff];
        let high = self.vram[(tile_addr | (r + 1)) & 0x1fff];
        (((high >> c) & 1) << 1) | ((low >> c) & 1)
    }

    fn get_tile_idx_from_tile_map(&self, tile_map: bool, y: u8, x: u8) -> usize {
        let start_addr: usize = 0x1800 | ((tile_map as usize) << 10);
        let ret = self.vram[start_addr | (((y as usize) << 5) + x as usize) & 0x3ff];
        if self.lcdc & BG_WINDOW_TILE_DATA_SELECT > 0 {
            ret as usize
        } else {
            ((ret as i8 as i16) + 0x100) as usize
        }
    }

    fn check_lyc_eq_ly(&mut self) {
        if self.ly == self.lyc {
            self.stat |= LYC_EQ_LY;
        } else {
            self.stat &= !LYC_EQ_LY;
        }
    }

    pub fn pixel_buffer(&self) -> Box<[u8]> {
        self.buffer
            .iter()
            .flat_map(|&e| iter::repeat(e.into()).take(3))
            .collect::<Box<[u8]>>()
    }

    fn render(&mut self) {
        if self.lcdc & BG_DISPLAY_ENABLE == 0 {
            return;
        }

        let y = self.ly.wrapping_add(self.scy);
        for i in 0..LCD_WIDTH {
            let x = (i as u8).wrapping_add(self.scx);
            let tile_ind =
                self.get_tile_idx_from_tile_map(self.lcdc & BG_TILE_MAP_SELECT > 0, y >> 3, x >> 3);

            let pixel = self.get_pixel_from_tile(tile_ind as usize, y & 7, x & 7);

            self.buffer[i + (self.ly as usize) * LCD_WIDTH] =
                match (self.bgp >> (pixel << 1)) & 0b11 {
                    0b00 => 0xff,
                    0b01 => 0xaa,
                    0b10 => 0x55,
                    _ => 0x00,
                };
        }
    }

    pub fn emu(&mut self) -> bool {
        // Check if PPU is enabled
        if self.lcdc & PPU_ENABLE == 0 {
            return false;
        }

        self.cycles -= 1;
        if self.cycles > 0 {
            return false;
        }

        let mut ret = false; // Is VSYNC
        match self.mode {
            Mode::OAMScan => {
                self.mode = Mode::Drawing;
                self.cycles = 43;
            }
            Mode::Drawing => {
                self.render();
                self.mode = Mode::HBlank;
                self.cycles = 51;
            }
            Mode::HBlank => {
                self.ly = self.ly.wrapping_add(1);
                if self.ly < 114 {
                    self.mode = Mode::OAMScan;
                    self.cycles = 20;
                } else {
                    self.mode = Mode::VBlank;
                    self.cycles = 114;
                }
                self.check_lyc_eq_ly();
            }
            Mode::VBlank => {
                self.ly = self.ly.wrapping_add(1);
                if self.ly > 153 {
                    ret = true;
                    self.ly = 0;
                    self.mode = Mode::OAMScan;
                    self.cycles = 20;
                } else {
                    self.cycles = 114;
                }
                self.check_lyc_eq_ly();
            }
        }
        ret
    }
}
