use crate::{
    instruction::{self, GBInstruction},
    reg::Registers,
};

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn execute(&mut self, instruction: GBInstruction) {
        match instruction {
            GBInstruction::ADD_A_A => self.add(TargetRegister::A),
            GBInstruction::ADD_A_B => self.add(TargetRegister::B),
            GBInstruction::ADD_A_C => self.add(TargetRegister::C),
            _ => unimplemented!("Instruction not implemented {:?}", instruction),
        }
    }

    fn add(&mut self, src_reg: TargetRegister) {
        let value = match src_reg {
            TargetRegister::A => self.registers.a,
            TargetRegister::B => self.registers.b,
            TargetRegister::C => self.registers.c,
            TargetRegister::D => self.registers.d,
            TargetRegister::E => self.registers.e,
            TargetRegister::H => self.registers.h,
            TargetRegister::L => self.registers.l,
        };

        let (res, _did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.a = res;
    }
}

enum TargetRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
