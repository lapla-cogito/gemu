use crate::{
    bootrom::Bootrom, cartridge::Cartridge, constants::M_CYCLE_NANOS, cpu::Cpu, lcd::Lcd,
    mem::Memory,
};
use sdl2;
use std::time;

pub struct Gameboy {
    cpu: Cpu,
    mem: Memory,
    lcd: Lcd,
}

impl Gameboy {
    pub fn new(bootrom: Bootrom, cartridge: Cartridge) -> Self {
        let sdl = sdl2::init().expect("failed to init SDL");
        let lcd = Lcd::new(&sdl, 4);

        let mem = Memory::new(bootrom, cartridge);
        let cpu = Cpu::new();

        Self { cpu, mem, lcd }
    }

    pub fn run(&mut self) {
        let time = time::Instant::now();
        let mut elapsed = 0;

        loop {
            let e = time.elapsed().as_nanos();

            for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
                self.cpu.emu(&mut self.mem);

                if self.mem.ppu.emu() {
                    self.lcd.draw(self.mem.ppu.pixel_buffer());
                }

                elapsed += M_CYCLE_NANOS;
            }
        }
    }
}
