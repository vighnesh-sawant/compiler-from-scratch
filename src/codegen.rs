use crate::ast::{BinOp, Expression, FunctionDeclaration, Program, Statement, UnOp};

pub fn generate(program: &Program) -> String {
    let mut asm = String::new();

    asm.push_str("    .intel_syntax noprefix\n");

    gen_function(&program.function, &mut asm);

    asm
}

fn gen_function(func: &FunctionDeclaration, asm: &mut String) {
    asm.push_str(&format!("    .globl {}\n", func.name));

    asm.push_str(&format!("{}:\n", func.name));

    gen_statement(&func.body, asm);
}

fn gen_statement(statement: &Statement, asm: &mut String) {
    match statement {
        Statement::Return(expr) => {
            gen_expression(expr, asm);

            asm.push_str("    ret\n");
        }
    }
}

fn gen_expression(expr: &Expression, asm: &mut String) {
    match expr {
        Expression::Constant(val) => {
            asm.push_str(&format!("    mov rax, {}\n", val));
        }
        
        Expression::UnaryOp((un_op,expression)) => {
            gen_expression(expression, asm);
            match un_op {
                UnOp::Negation => {
                    asm.push_str("    neg rax\n");
                }

                UnOp::BitwiseComplement=> {

                    asm.push_str("    not rax\n");

                }

                UnOp::LogicalNegation=> {

                    asm.push_str("    cmp rax, 0\n");
                    asm.push_str("    mov rax,0\n");
                    asm.push_str("    sete al\n");
                }
            } 

        }
        
        Expression::BinaryOp(binop,left ,right ) => {
            gen_expression(right, asm);
            match binop {
                BinOp::Add => {
                    asm.push_str("    push rax\n");
                    gen_expression(left, asm);
                    asm.push_str("    pop rcx\n");
                    asm.push_str("    add rax,rcx\n");
                }

                BinOp::Multiply=> {
                    asm.push_str("    push rax\n");
                    gen_expression(left, asm);
                    asm.push_str("    pop rcx\n");
                    asm.push_str("    imul rax,rcx\n");
                }

                BinOp::Subtract=> {
                    asm.push_str("    push rax\n");
                    gen_expression(left, asm);
                    asm.push_str("    pop rcx\n");
                    asm.push_str("    sub rax,rcx\n");
                }

                BinOp::Divide => {
                    asm.push_str("    push rax\n");
                    gen_expression(left, asm);
                    asm.push_str("    cqo\n");
                    asm.push_str("    pop rcx\n");
                    asm.push_str("    idiv rcx\n");
                }
            }

        }
    }
}
