use crate::ast;
use crate::ir::{self, Instruction, Val};

struct Generator {
    instructions: Vec<Instruction>,
    temp_counter: usize,
}

impl Generator {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            temp_counter: 0,
        }
    }

    fn make_temporary(&mut self) -> Val {
        let name = format!("temp.{}",self.temp_counter);
        self.temp_counter += 1;
        Val::Var(name)
    }


    fn emit_expression(&mut self, expr: &ast::Expression) -> Val {
        match expr {
            ast::Expression::Constant(c) => Val::Constant(*c),
            
            ast::Expression::UnaryOp((op, inner)) => {
                let src = self.emit_expression(inner);
                
                let dst = self.make_temporary();
                
                let tacky_op = match op {
                    ast::UnOp::Negation => ir::UnaryOp::Negation,
                    ast::UnOp::BitwiseComplement => ir::UnaryOp::BitwiseComplement,
                    ast::UnOp::LogicalNegation => ir::UnaryOp::LogicalNegation,
                };

                self.instructions.push(Instruction::Unary(tacky_op, src, dst.clone()));
                
                dst
            }

            ast::Expression::BinaryOp(op, left, right) => {
                let v1 = self.emit_expression(left);
                let v2 = self.emit_expression(right);
                
                let dst = self.make_temporary();

                let tacky_op = match op {
                    ast::BinOp::Add => ir::BinaryOp::Add,
                    ast::BinOp::Subtract => ir::BinaryOp::Subtract,
                    ast::BinOp::Multiply => ir::BinaryOp::Multiply,
                    ast::BinOp::Divide => ir::BinaryOp::Divide,
                    ast::BinOp::Remainder=> ir::BinaryOp::Remainder,
                };

                self.instructions.push(Instruction::Binary(tacky_op, v1, v2, dst.clone()));

                dst
            }
        }
    }

    fn emit_function(mut self, func: &ast::FunctionDeclaration) -> ir::Function {
        match &func.body {
            ast::Statement::Return(expr) => {
                let val = self.emit_expression(expr);
                self.instructions.push(Instruction::Return(val));
            }
        }

        ir::Function {
            name: func.name.clone(),
            instructions: self.instructions,
        }
    }
}

pub fn generate(program: &ast::Program) -> ir::Program {
    let generator = Generator::new();
    let function = generator.emit_function(&program.function);
    ir::Program { function }
}
