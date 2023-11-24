use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

use crate::{cpu::Cpu, mem::Memory};

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}
#[derive(Clone, Copy, Debug)]
pub struct Imm8;
#[derive(Clone, Copy, Debug)]
pub struct Imm16;
#[derive(Clone, Copy, Debug)]
pub enum Indirect {
    BC,
    DE,
    HL,
    CFF,
    HLD,
    HLI,
}
#[derive(Clone, Copy, Debug)]
pub enum Direct8 {
    D,
    DFF,
}
#[derive(Clone, Copy, Debug)]
pub struct Direct16;
#[derive(Clone, Copy, Debug)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

pub trait IO8<T: Copy> {
    fn read8(&mut self, mem: &Memory, src: T) -> Option<u8>;
    fn write8(&mut self, mem: &mut Memory, dst: T, val: u8) -> Option<()>;
}

pub trait IO16<T: Copy> {
    fn read16(&mut self, mem: &Memory, src: T) -> Option<u16>;
    fn write16(&mut self, mem: &mut Memory, dst: T, val: u16) -> Option<()>;
}

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _mem: &Memory, src: Reg8) -> Option<u8> {
        match src {
            Reg8::A => Some(self.regs.a),
            Reg8::B => Some(self.regs.b),
            Reg8::C => Some(self.regs.c),
            Reg8::D => Some(self.regs.d),
            Reg8::E => Some(self.regs.e),
            Reg8::H => Some(self.regs.h),
            Reg8::L => Some(self.regs.l),
        }
    }

    fn write8(&mut self, _mem: &mut Memory, dst: Reg8, val: u8) -> Option<()> {
        match dst {
            Reg8::A => {
                self.regs.a = val;
                Some(())
            }
            Reg8::B => {
                self.regs.b = val;
                Some(())
            }
            Reg8::C => {
                self.regs.c = val;
                Some(())
            }
            Reg8::D => {
                self.regs.d = val;
                Some(())
            }
            Reg8::E => {
                self.regs.e = val;
                Some(())
            }
            Reg8::H => {
                self.regs.h = val;
                Some(())
            }
            Reg8::L => {
                self.regs.l = val;
                Some(())
            }
        }
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _mem: &Memory, src: Reg16) -> Option<u16> {
        match src {
            Reg16::AF => Some(self.regs.af()),
            Reg16::BC => Some(self.regs.bc()),
            Reg16::DE => Some(self.regs.de()),
            Reg16::HL => Some(self.regs.hl()),
            Reg16::SP => Some(self.regs.sp),
        }
    }

    fn write16(&mut self, _mem: &mut Memory, dst: Reg16, val: u16) -> Option<()> {
        match dst {
            Reg16::AF => {
                self.regs.set_af(val);
                Some(())
            }
            Reg16::BC => {
                self.regs.set_bc(val);
                Some(())
            }
            Reg16::DE => {
                self.regs.set_de(val);
                Some(())
            }
            Reg16::HL => {
                self.regs.set_hl(val);
                Some(())
            }
            Reg16::SP => {
                self.regs.sp = val;
                Some(())
            }
        }
    }
}

impl IO8<Imm8> for Cpu {
    fn read8(&mut self, mem: &Memory, _src: Imm8) -> Option<u8> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);

        match STEP.load(Relaxed) {
            0 => {
                VALUE8.store(mem.read(self.regs.pc), Relaxed);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                STEP.store(1, Relaxed);
                None
            }
            1 => {
                STEP.store(0, Relaxed);
                Some(VALUE8.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write8(&mut self, _mem: &mut Memory, _dst: Imm8, _val: u8) -> Option<()> {
        unreachable!()
    }
}

impl IO16<Imm16> for Cpu {
    fn read16(&mut self, mem: &Memory, _src: Imm16) -> Option<u16> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);

        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(mem, Imm8) {
                    VALUE16.store(lo as u16, Relaxed);
                    STEP.store(1, Relaxed);
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(mem, Imm8) {
                    VALUE16.store(u16::from_le_bytes([VALUE8.load(Relaxed), hi]), Relaxed);
                    STEP.store(2, Relaxed);
                }
                None
            }
            2 => {
                STEP.store(0, Relaxed);
                Some(VALUE16.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    fn write16(&mut self, _mem: &mut Memory, _dst: Imm16, _val: u16) -> Option<()> {
        unreachable!()
    }
}

impl IO8<Indirect> for Cpu {
    fn read8(&mut self, mem: &Memory, src: Indirect) -> Option<u8> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VALUE8.store(
                    match src {
                        Indirect::BC => mem.read(self.regs.bc()),
                        Indirect::DE => mem.read(self.regs.de()),
                        Indirect::HL => mem.read(self.regs.hl()),
                        Indirect::CFF => mem.read(0xFF00 | u16::from(self.regs.c)),
                        Indirect::HLD => {
                            let addr = self.regs.hl();
                            self.regs.set_hl(addr.wrapping_sub(1));
                            mem.read(addr)
                        }
                        Indirect::HLI => {
                            let addr = self.regs.hl();
                            self.regs.set_hl(addr.wrapping_add(1));
                            mem.read(addr)
                        }
                    },
                    Relaxed,
                );
                STEP.store(1, Relaxed);
                None
            }
            1 => {
                STEP.store(0, Relaxed);
                Some(VALUE8.load(Relaxed))
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn write8(&mut self, mem: &mut Memory, dst: Indirect, val: u8) -> Option<()> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                match dst {
                    Indirect::BC => mem.write(self.regs.bc(), val),
                    Indirect::DE => mem.write(self.regs.de(), val),
                    Indirect::HL => mem.write(self.regs.hl(), val),
                    Indirect::CFF => mem.write(0xFF00 | u16::from(self.regs.c), val),
                    Indirect::HLD => {
                        let addr = self.regs.hl();
                        self.regs.set_hl(addr.wrapping_sub(1));
                        mem.write(addr, val)
                    }
                    Indirect::HLI => {
                        let addr = self.regs.hl();
                        self.regs.set_hl(addr.wrapping_add(1));
                        mem.write(addr, val)
                    }
                };
                STEP.store(1, Relaxed);
                None
            }
            1 => Some(STEP.store(0, Relaxed)),
            _ => {
                unreachable!()
            }
        }
    }
}

impl IO8<Direct8> for Cpu {
    fn read8(&mut self, mem: &Memory, src: Direct8) -> Option<u8> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(mem, Imm8) {
                    VALUE8.store(lo, Relaxed);
                    STEP.store(1, Relaxed);
                    if let Direct8::DFF = src {
                        VALUE16.store(0xFF00 | u16::from(lo), Relaxed);
                        STEP.store(2, Relaxed);
                    }
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(mem, Imm8) {
                    VALUE16.store(u16::from_le_bytes([VALUE8.load(Relaxed), hi]), Relaxed);
                    STEP.store(2, Relaxed);
                }
                None
            }
            2 => {
                VALUE8.store(mem.read(VALUE16.load(Relaxed)), Relaxed);
                STEP.store(3, Relaxed);
                None
            }
            3 => {
                STEP.store(0, Relaxed);
                Some(VALUE8.load(Relaxed))
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn write8(&mut self, mem: &mut Memory, dst: Direct8, val: u8) -> Option<()> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(mem, Imm8) {
                    VALUE8.store(lo, Relaxed);
                    STEP.store(1, Relaxed);
                    if let Direct8::DFF = dst {
                        VALUE16.store(0xFF00 | u16::from(lo), Relaxed);
                        STEP.store(2, Relaxed);
                    }
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(mem, Imm8) {
                    VALUE16.store(u16::from_le_bytes([VALUE8.load(Relaxed), hi]), Relaxed);
                    STEP.store(2, Relaxed);
                }
                None
            }
            2 => {
                mem.write(VALUE16.load(Relaxed), val);
                STEP.store(3, Relaxed);
                None
            }
            3 => Some(STEP.store(0, Relaxed)),
            _ => {
                unreachable!()
            }
        }
    }
}

impl IO16<Direct16> for Cpu {
    fn read16(&mut self, _mem: &Memory, _src: Direct16) -> Option<u16> {
        unreachable!()
    }

    fn write16(&mut self, mem: &mut Memory, _dst: Direct16, val: u16) -> Option<()> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(lo) = self.read8(mem, Imm8) {
                    VALUE8.store(lo, Relaxed);
                    STEP.store(1, Relaxed);
                }
                None
            }
            1 => {
                if let Some(hi) = self.read8(mem, Imm8) {
                    VALUE16.store(u16::from_le_bytes([VALUE8.load(Relaxed), hi]), Relaxed);
                    STEP.store(2, Relaxed);
                }
                None
            }
            2 => {
                mem.write(VALUE16.load(Relaxed), val as u8);
                STEP.store(3, Relaxed);
                None
            }
            3 => {
                mem.write(VALUE16.load(Relaxed).wrapping_add(1), (val >> 8) as u8);
                Some(STEP.store(4, Relaxed));
                None
            }
            4 => Some(STEP.store(0, Relaxed)),
            _ => {
                unreachable!()
            }
        }
    }
}
