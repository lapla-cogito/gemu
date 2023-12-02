use crate::{
    constants::VBLANK,
    cpu::{interrupts::Interrupts, reg::Registers},
    mem::Memory,
    JOYPAD, LCD_STAT, SERIAL, TIMER,
};

mod decode;
mod instructions;
mod interrupts;
mod operand;
mod reg;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
    int: bool,
}

pub struct Cpu {
    regs: Registers,
    ctx: Ctx,
    pub interrupts: Interrupts,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            ctx: Ctx::default(),
        }
    }

    pub fn emu(&mut self, mem: &mut Memory) {
        if self.ctx.int {
            self.call_isr(mem);
        } else {
            self.decode(mem);
        }
    }

    fn call_isr(&mut self, mem: &mut Memory) {
        step!(
            (),{
                0: if self.push16(mem,self.regs.pc).is_some(){
                    // Exec the highest priority interrupt
                    let highest=1<<self.interrupts.get_int().trailing_zeros();
                    self.interrupts.i_flag&=!highest;

                    self.regs.pc=match highest{
                        VBLANK=>0x40,
                        LCD_STAT=>0x48,
                        TIMER=>0x50,
                        SERIAL=>0x58,
                        JOYPAD=>0x60,
                        _=>panic!("Invalid interrupt: {:02x}",highest),
                    };
                    return go!(1);
                }
                1:{
                    self.interrupts.ime=false;
                    go!(0);
                    self.fetch(mem)
                }
            }
        );
    }

    pub fn fetch(&mut self, mem: &Memory) {
        let pc = self.regs.pc;
        let opcode = mem.read(pc);
        self.ctx.opcode = opcode;
        if self.interrupts.ime && self.interrupts.get_int() != 0 {
            self.ctx.int = true;
        } else {
            self.regs.pc = pc.wrapping_add(1);
            self.ctx.int = false;
        }
        self.ctx.cb = false;
    }
}
