use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

use crate::{
    cpu::{
        operand::{Cond, Imm16, Imm8, Reg16, IO16, IO8},
        Cpu,
    },
    mem::Memory,
};

impl Cpu {
    pub fn nop(&mut self, mem: &mut Memory) {
        self.fetch(mem);
    }

    pub fn ld<D: Copy, S: Copy>(&mut self, mem: &mut Memory, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(mem, src) {
                    VALUE8.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write8(mem, dst, VALUE8.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, mem: &mut Memory, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(mem, src) {
                    VALUE16.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write16(mem, dst, VALUE16.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn cp<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(v) = self.read8(mem, src) {
            let (res, carry) = self.regs.a.overflowing_sub(v);
            self.regs.set_zf(res == 0);
            self.regs.set_nf(true);
            self.regs.set_hf((self.regs.a & 0xf) < (v & 0xf));
            self.regs.set_cf(carry);
            self.fetch(mem);
        }
    }

    pub fn inc<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(mem, src) {
                    let res = v.wrapping_add(1);
                    self.regs.set_zf(res == 0);
                    self.regs.set_nf(false);
                    self.regs.set_hf((v & 0xf) == 0xf);
                    VALUE8.store(res, Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write8(mem, src, VALUE8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(mem);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn inc16<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO16<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(mem, src) {
                    VALUE16.store(v.wrapping_add(1), Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write16(mem, src, VALUE16.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn dec<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(mem, src) {
                    let res = v.wrapping_sub(1);
                    self.regs.set_zf(res == 0);
                    self.regs.set_nf(true);
                    self.regs.set_hf((v & 0xf) == 0);
                    VALUE8.store(res, Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write8(mem, src, VALUE8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(mem);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn dec16<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO16<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(mem, src) {
                    VALUE16.store(v.wrapping_sub(1), Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write16(mem, src, VALUE16.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn rl<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(mem, src) {
                    let res = (v << 1) | (self.regs.cf() as u8);
                    self.regs.set_zf(res == 0);
                    self.regs.set_nf(false);
                    self.regs.set_hf(false);
                    self.regs.set_cf(v & 0x80 > 0);
                    VALUE8.store(res, Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write8(mem, src, VALUE8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(mem);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn bit<S: Copy>(&mut self, mem: &mut Memory, bit: u8, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(mut v) = self.read8(mem, src) {
            v &= 1 << bit;
            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.fetch(mem);
        }
    }

    pub fn push16(&mut self, mem: &mut Memory, val: u16) -> Option<()> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                STEP.store(1, Relaxed);
                None
            }
            1 => {
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                mem.write(self.regs.sp, hi);
                VALUE8.store(lo, Relaxed);
                STEP.store(2, Relaxed);
                None
            }
            2 => {
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                mem.write(self.regs.sp, VALUE8.load(Relaxed));
                STEP.store(3, Relaxed);
                None
            }
            3 => Some(STEP.store(0, Relaxed)),
            _ => unreachable!(),
        }
    }

    pub fn push(&mut self, mem: &mut Memory, src: Reg16) {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VALUE16.store(self.read16(mem, src).unwrap(), Relaxed);
                STEP.store(1, Relaxed);
            }
            1 => {
                if self.push16(mem, VALUE16.load(Relaxed)).is_some() {
                    STEP.store(2, Relaxed);
                    self.fetch(mem);
                }
            }
            2 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn pop16(&mut self, mem: &mut Memory) -> Option<u16> {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VALUE8.store(mem.read(self.regs.sp), Relaxed);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                STEP.store(1, Relaxed);
                None
            }
            1 => {
                let hi = mem.read(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                VALUE16.store(u16::from_le_bytes([VALUE8.load(Relaxed), hi]), Relaxed);
                STEP.store(2, Relaxed);
                None
            }
            2 => {
                STEP.store(0, Relaxed);
                Some(VALUE16.load(Relaxed))
            }
            _ => unreachable!(),
        }
    }

    pub fn pop(&mut self, mem: &mut Memory, dst: Reg16) {
        if let Some(v) = self.pop16(mem) {
            self.write16(mem, dst, v);
            self.fetch(mem);
        }
    }

    pub fn jp(&mut self, mem: &mut Memory) {
        static STEP: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(mem, Imm16) {
                    self.regs.pc = v;
                    STEP.store(1, Relaxed)
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn jr(&mut self, mem: &mut Memory) {
        static STEP: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(mem, Imm8) {
                    self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                    STEP.store(1, Relaxed)
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn res<S: Copy>(&mut self, mem: &mut Memory, bit: usize, src: S)
    where
        Self: IO8<S>,
    {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(mem, src) {
                    VALUE8.store(v & !(1 << bit), Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.write8(mem, src, VALUE8.load(Relaxed)).is_some() {
                    STEP.store(0, Relaxed);
                    self.fetch(mem);
                }
            }
            _ => unreachable!(),
        }
    }

    fn cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.regs.zf(),
            Cond::Z => self.regs.zf(),
            Cond::NC => !self.regs.cf(),
            Cond::C => self.regs.cf(),
        }
    }

    pub fn jr_c(&mut self, mem: &mut Memory, cond: Cond) {
        static STEP: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read8(mem, Imm8) {
                    STEP.store(1, Relaxed);
                    if self.cond(cond) {
                        self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                        return;
                    }
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }

    pub fn call(&mut self, mem: &mut Memory) {
        static STEP: AtomicU16 = AtomicU16::new(0);
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.read16(mem, Imm16) {
                    VALUE16.store(v, Relaxed);
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                if self.push16(mem, self.regs.pc).is_some() {
                    self.regs.pc = VALUE16.load(Relaxed);
                    STEP.store(0, Relaxed);
                    self.fetch(mem);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn ret(&mut self, mem: &mut Memory) {
        static STEP: AtomicU16 = AtomicU16::new(0);
        match STEP.load(Relaxed) {
            0 => {
                if let Some(v) = self.pop16(mem) {
                    self.regs.pc = v;
                    STEP.store(1, Relaxed);
                }
            }
            1 => {
                STEP.store(0, Relaxed);
                self.fetch(mem);
            }
            _ => unreachable!(),
        }
    }
}
