use crate::{
    cpu::{
        operand::{Cond, Direct16, Direct8, Imm16, Imm8, Indirect, Reg16, Reg8, IO8},
        Cpu,
    },
    mem::Memory,
};

impl Cpu {
    pub fn decode(&mut self, mem: &mut Memory) {
        if self.ctx.cb {
            self.cb_decode(mem);
            return;
        }

        match self.ctx.opcode {
            0x00 => self.nop(mem),
            0x20 => self.jr_c(mem, Cond::NZ),
            0x30 => self.jr_c(mem, Cond::NC),
            0x01 => self.ld16(mem, Reg16::BC, Imm16),
            0x11 => self.ld16(mem, Reg16::DE, Imm16),
            0x21 => self.ld16(mem, Reg16::HL, Imm16),
            0x31 => self.ld16(mem, Reg16::SP, Imm16),
            0x02 => self.ld(mem, Indirect::BC, Reg8::A),
            0x12 => self.ld(mem, Indirect::DE, Reg8::A),
            0x22 => self.ld(mem, Indirect::HLI, Reg8::A),
            0x32 => self.ld(mem, Indirect::HLD, Reg8::A),
            0x03 => self.inc16(mem, Reg16::BC),
            0x13 => self.inc16(mem, Reg16::DE),
            0x23 => self.inc16(mem, Reg16::HL),
            0x33 => self.inc16(mem, Reg16::SP),
            0x04 => self.inc(mem, Reg8::B),
            0x14 => self.inc(mem, Reg8::D),
            0x24 => self.inc(mem, Reg8::H),
            0x34 => self.inc(mem, Indirect::HL),
            0x05 => self.dec(mem, Reg8::B),
            0x15 => self.dec(mem, Reg8::D),
            0x25 => self.dec(mem, Reg8::H),
            0x35 => self.dec(mem, Indirect::HL),
            0x06 => self.ld(mem, Reg8::B, Imm8),
            0x16 => self.ld(mem, Reg8::D, Imm8),
            0x26 => self.ld(mem, Reg8::H, Imm8),
            0x36 => self.ld(mem, Indirect::HL, Imm8),
            0x08 => self.ld16(mem, Direct16, Reg16::SP),
            0x18 => self.jr(mem),
            0x28 => self.jr_c(mem, Cond::Z),
            0x38 => self.jr_c(mem, Cond::C),
            0x0a => self.ld(mem, Reg8::A, Indirect::BC),
            0x1a => self.ld(mem, Reg8::A, Indirect::DE),
            0x2a => self.ld(mem, Reg8::A, Indirect::HLI),
            0x3a => self.ld(mem, Reg8::A, Indirect::HLD),
            0x0b => self.dec16(mem, Reg16::BC),
            0x1b => self.dec16(mem, Reg16::DE),
            0x2b => self.dec16(mem, Reg16::HL),
            0x3b => self.dec16(mem, Reg16::SP),
            0x0c => self.inc(mem, Reg8::C),
            0x1c => self.inc(mem, Reg8::E),
            0x2c => self.inc(mem, Reg8::L),
            0x3c => self.inc(mem, Reg8::A),
            0x0d => self.dec(mem, Reg8::C),
            0x1d => self.dec(mem, Reg8::E),
            0x2d => self.dec(mem, Reg8::L),
            0x3d => self.dec(mem, Reg8::A),
            0x0e => self.ld(mem, Reg8::C, Imm8),
            0x1e => self.ld(mem, Reg8::E, Imm8),
            0x2e => self.ld(mem, Reg8::L, Imm8),
            0x3e => self.ld(mem, Reg8::A, Imm8),
            0x40 => self.ld(mem, Reg8::B, Reg8::B),
            0x50 => self.ld(mem, Reg8::D, Reg8::B),
            0x60 => self.ld(mem, Reg8::H, Reg8::B),
            0x70 => self.ld(mem, Indirect::HL, Reg8::B),
            0x41 => self.ld(mem, Reg8::B, Reg8::C),
            0x51 => self.ld(mem, Reg8::D, Reg8::C),
            0x61 => self.ld(mem, Reg8::H, Reg8::C),
            0x71 => self.ld(mem, Indirect::HL, Reg8::C),
            0x42 => self.ld(mem, Reg8::B, Reg8::D),
            0x52 => self.ld(mem, Reg8::D, Reg8::D),
            0x62 => self.ld(mem, Reg8::H, Reg8::D),
            0x72 => self.ld(mem, Indirect::HL, Reg8::D),
            0x43 => self.ld(mem, Reg8::B, Reg8::E),
            0x53 => self.ld(mem, Reg8::D, Reg8::E),
            0x63 => self.ld(mem, Reg8::H, Reg8::E),
            0x73 => self.ld(mem, Indirect::HL, Reg8::E),
            0x44 => self.ld(mem, Reg8::B, Reg8::H),
            0x54 => self.ld(mem, Reg8::D, Reg8::H),
            0x64 => self.ld(mem, Reg8::H, Reg8::H),
            0x74 => self.ld(mem, Indirect::HL, Reg8::H),
            0x45 => self.ld(mem, Reg8::B, Reg8::L),
            0x55 => self.ld(mem, Reg8::D, Reg8::L),
            0x65 => self.ld(mem, Reg8::H, Reg8::L),
            0x75 => self.ld(mem, Indirect::HL, Reg8::L),
            0x46 => self.ld(mem, Reg8::B, Indirect::HL),
            0x56 => self.ld(mem, Reg8::D, Indirect::HL),
            0x66 => self.ld(mem, Reg8::H, Indirect::HL),
            0x47 => self.ld(mem, Reg8::B, Reg8::A),
            0x57 => self.ld(mem, Reg8::D, Reg8::A),
            0x67 => self.ld(mem, Reg8::H, Reg8::A),
            0x77 => self.ld(mem, Indirect::HL, Reg8::A),
            0x48 => self.ld(mem, Reg8::C, Reg8::B),
            0x58 => self.ld(mem, Reg8::E, Reg8::B),
            0x68 => self.ld(mem, Reg8::L, Reg8::B),
            0x78 => self.ld(mem, Reg8::A, Reg8::B),
            0x49 => self.ld(mem, Reg8::C, Reg8::C),
            0x59 => self.ld(mem, Reg8::E, Reg8::C),
            0x69 => self.ld(mem, Reg8::L, Reg8::C),
            0x79 => self.ld(mem, Reg8::A, Reg8::C),
            0x4a => self.ld(mem, Reg8::C, Reg8::D),
            0x5a => self.ld(mem, Reg8::E, Reg8::D),
            0x6a => self.ld(mem, Reg8::L, Reg8::D),
            0x7a => self.ld(mem, Reg8::A, Reg8::D),
            0x4b => self.ld(mem, Reg8::C, Reg8::E),
            0x5b => self.ld(mem, Reg8::E, Reg8::E),
            0x6b => self.ld(mem, Reg8::L, Reg8::E),
            0x7b => self.ld(mem, Reg8::A, Reg8::E),
            0x4c => self.ld(mem, Reg8::C, Reg8::H),
            0x5c => self.ld(mem, Reg8::E, Reg8::H),
            0x6c => self.ld(mem, Reg8::L, Reg8::H),
            0x7c => self.ld(mem, Reg8::A, Reg8::H),
            0x4d => self.ld(mem, Reg8::C, Reg8::L),
            0x5d => self.ld(mem, Reg8::E, Reg8::L),
            0x6d => self.ld(mem, Reg8::L, Reg8::L),
            0x7d => self.ld(mem, Reg8::A, Reg8::L),
            0x4e => self.ld(mem, Reg8::C, Indirect::HL),
            0x5e => self.ld(mem, Reg8::E, Indirect::HL),
            0x6e => self.ld(mem, Reg8::L, Indirect::HL),
            0x7e => self.ld(mem, Reg8::A, Indirect::HL),
            0x4f => self.ld(mem, Reg8::C, Reg8::A),
            0x5f => self.ld(mem, Reg8::E, Reg8::A),
            0x6f => self.ld(mem, Reg8::L, Reg8::A),
            0x7f => self.ld(mem, Reg8::A, Reg8::A),
            0xb8 => self.cp(mem, Reg8::B),
            0xb9 => self.cp(mem, Reg8::C),
            0xba => self.cp(mem, Reg8::D),
            0xbb => self.cp(mem, Reg8::E),
            0xbc => self.cp(mem, Reg8::H),
            0xbd => self.cp(mem, Reg8::L),
            0xbe => self.cp(mem, Indirect::HL),
            0xbf => self.cp(mem, Reg8::A),
            0xe0 => self.ld(mem, Direct8::DFF, Reg8::A),
            0xf0 => self.ld(mem, Reg8::A, Direct8::DFF),
            0xc1 => self.pop(mem, Reg16::BC),
            0xd1 => self.pop(mem, Reg16::DE),
            0xe1 => self.pop(mem, Reg16::HL),
            0xf1 => self.pop(mem, Reg16::AF),
            0xe2 => self.ld(mem, Indirect::CFF, Reg8::A),
            0xf2 => self.ld(mem, Reg8::A, Indirect::CFF),
            0xc3 => self.jp(mem),
            0xc5 => self.push(mem, Reg16::BC),
            0xd5 => self.push(mem, Reg16::DE),
            0xe5 => self.push(mem, Reg16::HL),
            0xf5 => self.push(mem, Reg16::AF),
            0xc9 => self.ret(mem),
            0xea => self.ld(mem, Direct8::D, Reg8::A),
            0xfa => self.ld(mem, Reg8::A, Direct8::D),
            0xcb => self.cb_prefixed(mem),
            0xcd => self.call(mem),
            0xfe => self.cp(mem, Imm8),
            _ => panic!("Unknown opcode: {:02X}", self.ctx.opcode),
        }
    }

    pub fn cb_decode(&mut self, mem: &mut Memory) {
        match self.ctx.opcode {
            0x10 => self.rl(mem, Reg8::B),
            0x11 => self.rl(mem, Reg8::C),
            0x12 => self.rl(mem, Reg8::D),
            0x13 => self.rl(mem, Reg8::E),
            0x14 => self.rl(mem, Reg8::H),
            0x15 => self.rl(mem, Reg8::L),
            0x16 => self.rl(mem, Indirect::HL),
            0x17 => self.rl(mem, Reg8::A),
            0x40 => self.bit(mem, 0, Reg8::B),
            0x50 => self.bit(mem, 2, Reg8::B),
            0x60 => self.bit(mem, 4, Reg8::B),
            0x70 => self.bit(mem, 6, Reg8::B),
            0x41 => self.bit(mem, 0, Reg8::C),
            0x51 => self.bit(mem, 2, Reg8::C),
            0x61 => self.bit(mem, 4, Reg8::C),
            0x71 => self.bit(mem, 6, Reg8::C),
            0x42 => self.bit(mem, 0, Reg8::D),
            0x52 => self.bit(mem, 2, Reg8::D),
            0x62 => self.bit(mem, 4, Reg8::D),
            0x72 => self.bit(mem, 6, Reg8::D),
            0x43 => self.bit(mem, 0, Reg8::E),
            0x53 => self.bit(mem, 2, Reg8::E),
            0x63 => self.bit(mem, 4, Reg8::E),
            0x73 => self.bit(mem, 6, Reg8::E),
            0x44 => self.bit(mem, 0, Reg8::H),
            0x54 => self.bit(mem, 2, Reg8::H),
            0x64 => self.bit(mem, 4, Reg8::H),
            0x74 => self.bit(mem, 6, Reg8::H),
            0x45 => self.bit(mem, 0, Reg8::L),
            0x55 => self.bit(mem, 2, Reg8::L),
            0x65 => self.bit(mem, 4, Reg8::L),
            0x75 => self.bit(mem, 6, Reg8::L),
            0x46 => self.bit(mem, 0, Indirect::HL),
            0x56 => self.bit(mem, 2, Indirect::HL),
            0x66 => self.bit(mem, 4, Indirect::HL),
            0x76 => self.bit(mem, 6, Indirect::HL),
            0x47 => self.bit(mem, 0, Reg8::A),
            0x57 => self.bit(mem, 2, Reg8::A),
            0x67 => self.bit(mem, 4, Reg8::A),
            0x77 => self.bit(mem, 6, Reg8::A),
            0x48 => self.bit(mem, 1, Reg8::B),
            0x58 => self.bit(mem, 3, Reg8::B),
            0x68 => self.bit(mem, 5, Reg8::B),
            0x78 => self.bit(mem, 7, Reg8::B),
            0x49 => self.bit(mem, 1, Reg8::C),
            0x59 => self.bit(mem, 3, Reg8::C),
            0x69 => self.bit(mem, 5, Reg8::C),
            0x79 => self.bit(mem, 7, Reg8::C),
            0x4a => self.bit(mem, 1, Reg8::D),
            0x5a => self.bit(mem, 3, Reg8::D),
            0x6a => self.bit(mem, 5, Reg8::D),
            0x7a => self.bit(mem, 7, Reg8::D),
            0x4b => self.bit(mem, 1, Reg8::E),
            0x5b => self.bit(mem, 3, Reg8::E),
            0x6b => self.bit(mem, 5, Reg8::E),
            0x7b => self.bit(mem, 7, Reg8::E),
            0x4c => self.bit(mem, 1, Reg8::H),
            0x5c => self.bit(mem, 3, Reg8::H),
            0x6c => self.bit(mem, 5, Reg8::H),
            0x7c => self.bit(mem, 7, Reg8::H),
            0x4d => self.bit(mem, 1, Reg8::L),
            0x5d => self.bit(mem, 3, Reg8::L),
            0x6d => self.bit(mem, 5, Reg8::L),
            0x7d => self.bit(mem, 7, Reg8::L),
            0x4e => self.bit(mem, 1, Indirect::HL),
            0x5e => self.bit(mem, 3, Indirect::HL),
            0x6e => self.bit(mem, 5, Indirect::HL),
            0x7e => self.bit(mem, 7, Indirect::HL),
            0x4f => self.bit(mem, 1, Reg8::A),
            0x5f => self.bit(mem, 3, Reg8::A),
            0x6f => self.bit(mem, 5, Reg8::A),
            0x7f => self.bit(mem, 7, Reg8::A),
            0x80 => self.res(mem, 0, Reg8::B),
            0x90 => self.res(mem, 2, Reg8::B),
            0xa0 => self.res(mem, 4, Reg8::B),
            0xb0 => self.res(mem, 6, Reg8::B),
            0x81 => self.res(mem, 0, Reg8::C),
            0x91 => self.res(mem, 2, Reg8::C),
            0xa1 => self.res(mem, 4, Reg8::C),
            0xb1 => self.res(mem, 6, Reg8::C),
            0x82 => self.res(mem, 0, Reg8::D),
            0x92 => self.res(mem, 2, Reg8::D),
            0xa2 => self.res(mem, 4, Reg8::D),
            0xb2 => self.res(mem, 6, Reg8::D),
            0x83 => self.res(mem, 0, Reg8::E),
            0x93 => self.res(mem, 2, Reg8::E),
            0xa3 => self.res(mem, 4, Reg8::E),
            0xb3 => self.res(mem, 6, Reg8::E),
            0x84 => self.res(mem, 0, Reg8::H),
            0x94 => self.res(mem, 2, Reg8::H),
            0xa4 => self.res(mem, 4, Reg8::H),
            0xb4 => self.res(mem, 6, Reg8::H),
            0x85 => self.res(mem, 0, Reg8::L),
            0x95 => self.res(mem, 2, Reg8::L),
            0xa5 => self.res(mem, 4, Reg8::L),
            0xb5 => self.res(mem, 6, Reg8::L),
            0x86 => self.res(mem, 0, Indirect::HL),
            0x96 => self.res(mem, 2, Indirect::HL),
            0xa6 => self.res(mem, 4, Indirect::HL),
            0xb6 => self.res(mem, 6, Indirect::HL),
            0x87 => self.res(mem, 0, Reg8::A),
            0x97 => self.res(mem, 2, Reg8::A),
            0xa7 => self.res(mem, 4, Reg8::A),
            0xb7 => self.res(mem, 6, Reg8::A),
            0x88 => self.res(mem, 1, Reg8::B),
            0x98 => self.res(mem, 3, Reg8::B),
            0xa8 => self.res(mem, 5, Reg8::B),
            0xb8 => self.res(mem, 7, Reg8::B),
            0x89 => self.res(mem, 1, Reg8::C),
            0x99 => self.res(mem, 3, Reg8::C),
            0xa9 => self.res(mem, 5, Reg8::C),
            0xb9 => self.res(mem, 7, Reg8::C),
            0x8a => self.res(mem, 1, Reg8::D),
            0x9a => self.res(mem, 3, Reg8::D),
            0xaa => self.res(mem, 5, Reg8::D),
            0xba => self.res(mem, 7, Reg8::D),
            0x8b => self.res(mem, 1, Reg8::E),
            0x9b => self.res(mem, 3, Reg8::E),
            0xab => self.res(mem, 5, Reg8::E),
            0xbb => self.res(mem, 7, Reg8::E),
            0x8c => self.res(mem, 1, Reg8::H),
            0x9c => self.res(mem, 3, Reg8::H),
            0xac => self.res(mem, 5, Reg8::H),
            0xbc => self.res(mem, 7, Reg8::H),
            0x8d => self.res(mem, 1, Reg8::L),
            0x9d => self.res(mem, 3, Reg8::L),
            0xad => self.res(mem, 5, Reg8::L),
            0xbd => self.res(mem, 7, Reg8::L),
            0x8e => self.res(mem, 1, Indirect::HL),
            0x9e => self.res(mem, 3, Indirect::HL),
            0xAe => self.res(mem, 5, Indirect::HL),
            0xBe => self.res(mem, 7, Indirect::HL),
            0x8f => self.res(mem, 1, Reg8::A),
            0x9f => self.res(mem, 3, Reg8::A),
            0xaf => self.res(mem, 5, Reg8::A),
            0xbf => self.res(mem, 7, Reg8::A),
            _ => panic!("Unknown opcode: {:02X}", self.ctx.opcode),
        }
    }

    fn cb_prefixed(&mut self, mem: &mut Memory) {
        if let Some(v) = self.read8(mem, Imm8) {
            self.ctx.opcode = v;
            self.ctx.cb = true;
            self.cb_decode(mem);
        }
    }
}
