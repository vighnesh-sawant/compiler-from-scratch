use crate::ast::{BinOp, Expression, FunctionDeclaration, Program as AstProgram, Statement, UnOp};
use crate::ir::{
    Function as AsmFunction, Instruction::{self, *}, Operand::*, Program as AsmProgram, Reg::*,
};

pub fn generate(program: &AstProgram) -> AsmProgram {
    let function = gen_function(&program.function);
    AsmProgram { function }
}

fn gen_function(func: &FunctionDeclaration) -> AsmFunction {
    let mut instructions = Vec::new();

    instructions.extend(gen_statement(&func.body));

    AsmFunction {
        name: func.name.clone(),
        instructions,
    }
}

fn gen_statement(statement: &Statement) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    match statement {
        Statement::Return(expr) => {
            instructions.extend(gen_expression(expr));
            instructions.push(Ret);
        }
    }
    instructions
}

fn gen_expression(expr: &Expression) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    match expr {
        Expression::Constant(val) => {
            // mov rax, val
            instructions.push(Mov(Reg(RAX), Imm(*val)));
        }

        Expression::UnaryOp((un_op, expression)) => {
            instructions.extend(gen_expression(expression));
            match un_op {
                UnOp::Negation => {
                    // neg rax
                    instructions.push(Neg(Reg(RAX)));
                }
                UnOp::BitwiseComplement => {
                    // not rax
                    instructions.push(Not(Reg(RAX)));
                }
                UnOp::LogicalNegation => {
                    // cmp rax, 0
                    instructions.push(Cmp(Reg(RAX), Imm(0)));
                    // mov rax, 0 (Zero out RAX)
                    instructions.push(Mov(Reg(RAX), Imm(0)));
                    // sete al (Set lower byte to 1 if equal, 0 otherwise)
                    instructions.push(Sete(Reg(AL)));
                }
            }
        }

        Expression::BinaryOp(binop, left, right) => {
            // Eval Right -> Push -> Eval Left -> Pop -> Op

            //  Evaluate Right
            instructions.extend(gen_expression(right));

            //  Push result (RAX) to stack
            instructions.push(Push(Reg(RAX)));

            //  Evaluate Left
            instructions.extend(gen_expression(left));

            //  Pop previous result into RCX
            instructions.push(Pop(Reg(RCX)));

            //  Perform Operation
            match binop {
                BinOp::Add => {
                    // add rax, rcx
                    instructions.push(Add(Reg(RAX), Reg(RCX)));
                }
                BinOp::Multiply => {
                    // imul rax, rcx
                    instructions.push(Imul(Reg(RAX), Reg(RCX)));
                }
                BinOp::Subtract => {
                    // sub rax, rcx
                    instructions.push(Sub(Reg(RAX), Reg(RCX)));
                }
                BinOp::Divide => {
                    // cqo (Sign extend RAX for division)
                    instructions.push(Cqo);
                    // idiv rcx
                    instructions.push(Idiv(Reg(RCX)));
                }
            }
        }
    }

    instructions
}
