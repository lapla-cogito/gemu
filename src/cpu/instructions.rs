use crate::{
    cpu::operand::{Cond, Imm16, Imm8, Reg16, IO16, IO8},
    cpu::Cpu,
    mem::Memory,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

macro_rules! step {
    ($d: expr, {$($c:tt : $e:expr,)*}) => {
        static STEP: AtomicU8 = AtomicU8::new(0);
        #[allow(unused)]
        static VALUE8: AtomicU8 = AtomicU8::new(0);
        #[allow(unused)]
        static VALUE16: AtomicU16 = AtomicU16::new(0);
        $(if STEP.load(Relaxed) == $c { $e })* else { return $d; }
    };
}
pub(crate) use step;

macro_rules! go {
    ($e:expr) => {
        STEP.store($e, Relaxed)
    };
}
pub(crate) use go;

impl Cpu {
    pub fn nop(&mut self, mem: &Memory) {
        self.fetch(mem);
    }

    pub fn ld<D: Copy, S: Copy>(&mut self, mem: &mut Memory, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        step!((), {
            0: if let Some(v) = self.read8(mem,src) {
                VALUE8.store(v, Relaxed);
                go!(1);
            },
            1: if self.write8(mem, dst, VALUE8.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, mem: &mut Memory, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        step!((), {
            0: if let Some(v) = self.read16(mem, src) {
                VALUE16.store(v, Relaxed);
                go!(1);
            },
            1: if self.write16(mem, dst, VALUE16.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn res<S: Copy>(&mut self, mem: &mut Memory, bit: usize, src: S)
    where
        Self: IO8<S>,
    {
        step!((), {
            0: if let Some(v) = self.read8(mem, src) {
                VALUE8.store(v & !(1 << bit), Relaxed);
                go!(1);
            },
            1: if self.write8(mem, src, VALUE8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn jp(&mut self, mem: &Memory) {
        step!((), {
            0: if let Some(v) = self.read16(mem, Imm16) {
                self.regs.pc = v;
                return go!(1);
            },
            1: {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn cp<S: Copy>(&mut self, mem: &Memory, src: S)
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
        step!((), {
            0: if let Some(v) = self.read8(mem, src) {
                let res = v.wrapping_add(1);
                self.regs.set_zf(res == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(v & 0xf == 0xf);
                VALUE8.store(res, Relaxed);
                go!(1);
            },
            1: if self.write8(mem, src, VALUE8.load(Relaxed)).is_some(){
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn inc16<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO16<S>,
    {
        step!((), {
            0: if let Some(v) = self.read16(mem, src) {
                VALUE16.store(v.wrapping_add(1), Relaxed);
                go!(1);
            },
            1: if self.write16(mem, src, VALUE16.load(Relaxed)).is_some(){
                return go!(2);
            },
            2: {
                go!(0);
                self.fetch(mem)
            },
        });
    }

    pub fn dec<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO8<S>,
    {
        step!((), {
            0: if let Some(v) = self.read8(mem, src) {
                let result = v.wrapping_sub(1);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(true);
                self.regs.set_hf(v & 0xf == 0);
                VALUE8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(mem, src, VALUE8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn dec16<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO16<S>,
    {
        step!((), {
            0: if let Some(v) = self.read16(mem, src) {
                VALUE16.store(v.wrapping_sub(1), Relaxed);
                go!(1);
            },
            1: if self.write16(mem, src, VALUE16.load(Relaxed)).is_some() {
                return go!(2);
            },
            2: {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn rl<S: Copy>(&mut self, mem: &mut Memory, src: S)
    where
        Self: IO8<S>,
    {
        step!((), {
            0: if let Some(v) = self.read8(mem, src) {
                let res = (v << 1) | self.regs.cf() as u8;
                self.regs.set_zf(res == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0x80 > 0);
                VALUE8.store(res, Relaxed);
                go!(1);
            },
            1: if self.write8(mem, src, VALUE8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn bit<S: Copy>(&mut self, mem: &Memory, bit: usize, src: S)
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
        step!(None, {
            0: {
                go!(1);
                return None;
            },
            1: {
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                mem.write(self.regs.sp, hi);
                VALUE8.store(lo, Relaxed);
                go!(2);
                return None;
            },
            2: {
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                mem.write(self.regs.sp, VALUE8.load(Relaxed));
                go!(3);
                return None;
            },
            3: return Some(go!(0)),
        });
    }

    pub fn push(&mut self, mem: &mut Memory, src: Reg16) {
        step!((), {
            0: {
                VALUE16.store(self.read16(mem, src).unwrap(), Relaxed);
                go!(1);
            },
            1: if self.push16(mem, VALUE16.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn pop16(&mut self, mem: &Memory) -> Option<u16> {
        step!(None, {
            0: {
                VALUE8.store(mem.read(self.regs.sp), Relaxed);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                go!(1);
                return None;
            },
            1: {
                let hi = mem.read(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                VALUE16.store(u16::from_le_bytes([VALUE8.load(Relaxed), hi]), Relaxed);
                go!(2);
                return None;
            },
            2: {
                go!(0);
                return Some(VALUE16.load(Relaxed));
            },
        });
    }

    pub fn pop(&mut self, mem: &mut Memory, dst: Reg16) {
        if let Some(v) = self.pop16(mem) {
            self.write16(mem, dst, v);
            self.fetch(mem);
        }
    }

    pub fn jr(&mut self, mem: &Memory) {
        step!((), {
            0: if let Some(v) = self.read8(mem, Imm8) {
                self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                return go!(1);
            },
            1: {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    fn cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.regs.zf(),
            Cond::Z => self.regs.zf(),
            Cond::NC => !self.regs.cf(),
            Cond::C => self.regs.cf(),
        }
    }

    pub fn jr_c(&mut self, mem: &Memory, c: Cond) {
        step!((), {
            0: if let Some(v) = self.read8(mem, Imm8) {
                go!(1);
                if self.cond(c) {
                    self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                    return;
                }
            },
            1: {
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn call(&mut self, mem: &mut Memory) {
        step!((), {
            0: if let Some(v) = self.read16(mem, Imm16) {
                VALUE16.store(v, Relaxed);
                go!(1);
            },
            1: if self.push16(mem, self.regs.pc).is_some() {
                self.regs.pc = VALUE16.load(Relaxed);
                go!(0);
                self.fetch(mem);
            },
        });
    }

    pub fn ret(&mut self, mem: &Memory) {
        step!((), {
            0: if let Some(v) = self.pop16(mem) {
                self.regs.pc = v;
                go!(1);
            },
            1: {
                go!(0);
                self.fetch(mem);
            },
        });
    }
}
