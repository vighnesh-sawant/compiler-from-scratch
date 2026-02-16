use std::{collections::HashMap, fmt};

use crate::ir::{self};

#[derive(Debug, Clone, PartialEq)]
pub enum Reg {
    Rax,
    Rcx,
    Rdx,
    R10,
    R11,
    Rsp,
    Rbp, // 64-bit
    Al,
    Cl,
    Dl,
    R10b,
    R11b,
    Spl,
    Bpl, // 8-bit (Low Byte)
}
impl Reg {
    pub fn to_byte_reg(&self) -> Reg {
        match self {
            // 64-bit -> 8-bit (Low Byte)
            Reg::Rax => Reg::Al,
            Reg::Rcx => Reg::Cl,
            Reg::Rdx => Reg::Dl,
            Reg::R10 => Reg::R10b,
            Reg::R11 => Reg::R11b,
            Reg::Rsp => Reg::Spl,
            Reg::Rbp => Reg::Bpl,

            // Already 8-bit (Identity)
            Reg::Al => Reg::Al,
            Reg::Cl => Reg::Cl,
            Reg::Dl => Reg::Dl,
            Reg::R10b => Reg::R10b,
            Reg::R11b => Reg::R11b,
            Reg::Spl => Reg::Spl,
            Reg::Bpl => Reg::Bpl,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Imm(i32),
    Reg(Reg),
    Pseudo(String),
    StackQWord(i32),
    StackByte(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Mov(Operand, Operand),  // mov dst, src
    Add(Operand, Operand),  // add dst, src
    Sub(Operand, Operand),  // sub dst, src
    Imul(Operand, Operand), // imul dst, src
    Idiv(Operand),          // idiv src (implicit rax/rdx)

    // Unary
    Neg(Operand), // neg dst
    Not(Operand), // not dst

    // Stack
    Push(Operand),
    Pop(Operand),

    // Control / Comparison
    Ret,
    Cmp(Operand, Operand),
    Sete(Operand), // sete dst
    Cqo,           // Sign extend rax into rdx:rax
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // 64-bit registers
            Reg::Rax => write!(f, "rax"),
            Reg::Rcx => write!(f, "rcx"),
            Reg::Rdx => write!(f, "rdx"),
            Reg::R10 => write!(f, "r10"),
            Reg::R11 => write!(f, "r11"),
            Reg::Rsp => write!(f, "rsp"),
            Reg::Rbp => write!(f, "rbp"),

            // 8-bit registers (Low Byte)
            Reg::Al => write!(f, "al"),
            Reg::Cl => write!(f, "cl"),
            Reg::Dl => write!(f, "dl"),
            Reg::R10b => write!(f, "r10b"),
            Reg::R11b => write!(f, "r11b"),
            Reg::Spl => write!(f, "spl"),
            Reg::Bpl => write!(f, "bpl"),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Imm(val) => write!(f, "{}", val),
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::StackQWord(offset) => write!(f, "QWORD PTR [rbp{}]", offset),
            Operand::StackByte(offset) => write!(f, "BYTE PTR [rbp{}]", offset),
            Operand::Pseudo(_) => unreachable!(),
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

fn select_instructions(ir_fn: &ir::Function) -> Vec<Instruction> {
    let mut insts = Vec::new();

    for instruction in &ir_fn.instructions {
        match instruction {
            ir::Instruction::Binary(op, src1, src2, dst) => {
                let s1 = to_operand(src1);
                let s2 = to_operand(src2);
                let d = to_operand(dst);

                match op {
                    ir::BinaryOp::Add => {
                        insts.push(Instruction::Mov(d.clone(), s1));
                        insts.push(Instruction::Add(d, s2));
                    }
                    ir::BinaryOp::Subtract => {
                        insts.push(Instruction::Mov(d.clone(), s1));
                        insts.push(Instruction::Sub(d, s2));
                    }
                    ir::BinaryOp::Multiply => {
                        insts.push(Instruction::Mov(d.clone(), s1));
                        insts.push(Instruction::Imul(d, s2));
                    }

                    // Division / Remainder
                    ir::BinaryOp::Divide | ir::BinaryOp::Remainder => {
                        insts.push(Instruction::Mov(Operand::Reg(Reg::Rax), s1));
                        insts.push(Instruction::Cqo); // Sign extend RAX -> RDX:RAX
                        insts.push(Instruction::Idiv(s2));

                        let result_reg = match op {
                            ir::BinaryOp::Divide => Reg::Rax,
                            ir::BinaryOp::Remainder => Reg::Rdx,
                            _ => unreachable!(),
                        };
                        insts.push(Instruction::Mov(d, Operand::Reg(result_reg)));
                    }
                }
            }

            ir::Instruction::Unary(op, src, dst) => {
                let s = to_operand(src);
                let d = to_operand(dst);

                match op {
                    ir::UnaryOp::Negation => {
                        insts.push(Instruction::Mov(d.clone(), s));
                        insts.push(Instruction::Neg(d));
                    }
                    ir::UnaryOp::BitwiseComplement => {
                        insts.push(Instruction::Mov(d.clone(), s));
                        insts.push(Instruction::Not(d));
                    }
                    ir::UnaryOp::LogicalNegation => {
                        if let Operand::Imm(val) = &s {
                            if *val == 0 {
                                insts.push(Instruction::Mov(d.clone(), Operand::Imm(1)));
                            } else {
                                insts.push(Instruction::Mov(d.clone(), Operand::Imm(0)));
                            }
                        } else {
                            insts.push(Instruction::Cmp(s, Operand::Imm(0)));

                            insts.push(Instruction::Mov(d.clone(), Operand::Imm(0)));

                            insts.push(Instruction::Sete(d));
                        }
                    }
                }
            }

            ir::Instruction::Return(val) => {
                insts.push(Instruction::Mov(Operand::Reg(Reg::Rax), to_operand(val)));
                insts.push(Instruction::Ret);
            }
        }
    }
    insts
}

fn allocate_stack(insts: Vec<Instruction>) -> (Vec<Instruction>, i32) {
    let mut map = HashMap::new();
    let mut stack_size = 0;
    let mut new_insts = Vec::new();

    let mut replace_operand = |op: &Operand| -> Operand {
        if let Operand::Pseudo(name) = op {
            if !map.contains_key(name) {
                stack_size -= 8; // Allocate 8 bytes
                map.insert(name.clone(), stack_size);
            }
            Operand::StackQWord(map[name])
        } else {
            op.clone()
        }
    };

    for inst in insts {
        // Reconstruct instruction with replaced operands
        let new_inst = match inst {
            Instruction::Mov(dst, src) => {
                Instruction::Mov(replace_operand(&dst), replace_operand(&src))
            }
            Instruction::Add(dst, src) => {
                Instruction::Add(replace_operand(&dst), replace_operand(&src))
            }
            Instruction::Sub(dst, src) => {
                Instruction::Sub(replace_operand(&dst), replace_operand(&src))
            }
            Instruction::Imul(dst, src) => {
                Instruction::Imul(replace_operand(&dst), replace_operand(&src))
            }
            Instruction::Cmp(dst, src) => {
                Instruction::Cmp(replace_operand(&dst), replace_operand(&src))
            }

            Instruction::Idiv(op) => Instruction::Idiv(replace_operand(&op)),
            Instruction::Neg(op) => Instruction::Neg(replace_operand(&op)),
            Instruction::Not(op) => Instruction::Not(replace_operand(&op)),
            Instruction::Push(op) => Instruction::Push(replace_operand(&op)),
            Instruction::Pop(op) => Instruction::Pop(replace_operand(&op)),
            Instruction::Sete(op) => Instruction::Sete(replace_operand(&op)),

            Instruction::Ret => Instruction::Ret,
            Instruction::Cqo => Instruction::Cqo,
        };
        new_insts.push(new_inst);
    }

    (new_insts, stack_size)
}
fn fix_instructions(insts: Vec<Instruction>) -> Vec<Instruction> {
    let mut clean_insts = Vec::new();

    for inst in insts {
        match inst {
            Instruction::Add(ref dst, ref src)
            | Instruction::Sub(ref dst, ref src)
            | Instruction::Cmp(ref dst, ref src) => {
                // Check if BOTH are stack locations
                if let (Operand::StackQWord(_), Operand::StackQWord(_)) = (&dst, &src) {
                    // Rewrite: add [mem], [mem]  ->  mov r10, [mem]; add r10, [mem]; mov [mem], r10

                    // Load dst into scratch register R10
                    clean_insts.push(Instruction::Mov(Operand::Reg(Reg::R10), src.clone()));

                    //  Perform operation on R10
                    // (Construct the same instruction type but with R10)
                    let new_op = match inst {
                        Instruction::Add(_, _) => {
                            Instruction::Add(dst.clone(), Operand::Reg(Reg::R10))
                        }
                        Instruction::Sub(_, _) => {
                            Instruction::Sub(dst.clone(), Operand::Reg(Reg::R10))
                        }

                        Instruction::Cmp(_, _) => {
                            Instruction::Cmp(dst.clone(), Operand::Reg(Reg::R10))
                        }
                        _ => unreachable!(),
                    };
                    clean_insts.push(new_op);

                } else {
                    clean_insts.push(inst);
                }
            }

            // Case: Mov (Cannot move Mem to Mem)
            Instruction::Mov(ref dst, ref src) => {
                if let (Operand::StackQWord(_), Operand::StackQWord(_)) = (&dst, &src) {
                    // Rewrite: mov [dst], [src] -> mov r10, [src]; mov [dst], r10
                    clean_insts.push(Instruction::Mov(Operand::Reg(Reg::R10), src.clone()));
                    clean_insts.push(Instruction::Mov(dst.clone(), Operand::Reg(Reg::R10)));
                } else {
                    clean_insts.push(inst);
                }
            }

            Instruction::Sete(ref dst) => {
                let inst = match dst {
                    Operand::Reg(reg) => Instruction::Sete(Operand::Reg(reg.to_byte_reg())),
                    Operand::StackQWord(offset) => Instruction::Sete(Operand::StackByte(*offset)),
                    _ => unreachable!(),
                };
                clean_insts.push(inst);
            }

            Instruction::Idiv(ref src) => {
                if let Operand::Imm(val) = src {
                    clean_insts.push(Instruction::Mov(Operand::Reg(Reg::R10), Operand::Imm(*val)));
                    clean_insts.push(Instruction::Idiv(Operand::Reg(Reg::R10)));
                } else {
                    clean_insts.push(inst);
                }
            }
            Instruction::Imul(ref dst, ref src) => {
                if let Operand::StackQWord(_) = dst {
                    clean_insts.push(Instruction::Mov(Operand::Reg(Reg::R11), dst.clone()));

                    clean_insts.push(Instruction::Imul(Operand::Reg(Reg::R11), src.clone()));

                    clean_insts.push(Instruction::Mov(dst.clone(), Operand::Reg(Reg::R11)));
                } else {
                    clean_insts.push(Instruction::Imul(dst.clone(), src.clone()));
                }
            }

            // Pass through others
            _ => clean_insts.push(inst),
        }
    }

    clean_insts
}
fn to_operand(v: &ir::Val) -> Operand {
    match v {
        ir::Val::Constant(i) => Operand::Imm(*i),
        ir::Val::Var(s) => Operand::Pseudo(s.clone()),
    }
}

pub fn generate(program: &ir::Program) -> Program {
    let abstract_asm = select_instructions(&program.function);

    let (stack_asm, stack_size) = allocate_stack(abstract_asm);

    let mut valid_asm = fix_instructions(stack_asm);

    // Insert Prologue/Epilogue
    valid_asm.insert(0, Instruction::Push(Operand::Reg(Reg::Rbp)));
    valid_asm.insert(
        1,
        Instruction::Mov(Operand::Reg(Reg::Rbp), Operand::Reg(Reg::Rsp)),
    );

    if stack_size != 0 {
        let alignment = 16;
        let aligned_size = ((-stack_size + alignment - 1) / alignment) * alignment;
        valid_asm.insert(
            2,
            Instruction::Sub(Operand::Reg(Reg::Rsp), Operand::Imm(aligned_size)),
        );
        valid_asm.insert(
            valid_asm.len() - 1,
            Instruction::Mov(Operand::Reg(Reg::Rsp), Operand::Reg(Reg::Rbp)),
        );
    }
    valid_asm.insert(
        valid_asm.len() - 1,
        Instruction::Pop(Operand::Reg(Reg::Rbp)),
    );

    Program {
        function: Function {
            name: program.function.name.clone(),
            instructions: valid_asm,
        },
    }
}
