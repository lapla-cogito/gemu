use crate::{cpu::reg::Registers, mem::Memory};

mod decode;
mod instructions;
mod operand;
mod reg;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    pub regs: Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            ctx: Ctx::default(),
        }
    }

    pub fn emu(&mut self, mem: &mut Memory) {
        self.decode(mem);
    }

    pub fn fetch(&mut self, mem: &Memory) {
        let pc = self.regs.pc;
        let opcode = mem.read(pc);
        self.ctx.opcode = opcode;
        self.regs.pc = pc.wrapping_add(1);
        self.ctx.cb = false;
    }
}
