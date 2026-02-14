use std::fmt;
use crate::ir::{Program, Function, Instruction, Operand, Reg};


impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reg::RAX => write!(f, "rax"),
            Reg::RCX => write!(f, "rcx"),
            Reg::AL => write!(f, "al"),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Imm(val) => write!(f, "{}", val),
            Operand::Reg(reg) => write!(f, "{}", reg), 
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Mov(dst, src) => write!(f, "    mov {}, {}", dst, src),
            Instruction::Add(dst, src) => write!(f, "    add {}, {}", dst, src),
            Instruction::Sub(dst, src) => write!(f, "    sub {}, {}", dst, src),
            Instruction::Imul(dst, src) => write!(f, "    imul {}, {}", dst, src),
            Instruction::Cmp(dst, src) => write!(f, "    cmp {}, {}", dst, src),

            Instruction::Idiv(op) => write!(f, "    idiv {}", op),
            Instruction::Neg(op) => write!(f, "    neg {}", op),
            Instruction::Not(op) => write!(f, "    not {}", op),
            Instruction::Push(op) => write!(f, "    push {}", op),
            Instruction::Pop(op) => write!(f, "    pop {}", op),
            Instruction::Sete(op) => write!(f, "    sete {}", op),

            Instruction::Ret => write!(f, "    ret"),
            Instruction::Cqo => write!(f, "    cqo"),
        }
    }
}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;

        for instr in &self.instructions {
            writeln!(f, "{}", instr)?; 
        }
        
        Ok(())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "    .intel_syntax noprefix")?;
        
        writeln!(f, "    .globl {}", self.function.name)?;

        write!(f, "{}", self.function)?; 

        writeln!(f, "    .section .note.GNU-stack,\"\",@progbits")
    }
}
