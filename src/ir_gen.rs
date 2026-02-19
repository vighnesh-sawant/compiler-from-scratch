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
        let name = format!("temp.{}", self.temp_counter);
        self.temp_counter += 1;
        Val::Var(name)
    }

    fn make_label(&mut self, label: &str) -> String {
        let name = format!(".L{}.{}", label, self.temp_counter);
        self.temp_counter += 1;
        name
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

                self.instructions
                    .push(Instruction::Unary(tacky_op, src, dst.clone()));

                dst
            }

            ast::Expression::BinaryOp(op, left, right) => match op {
                ast::BinOp::LogicalAnd => {
                    let dst = self.make_temporary();
                    let false_label = self.make_label("and_false");
                    let end_label = self.make_label("and_end");

                    let v1 = self.emit_expression(left);
                    self.instructions
                        .push(Instruction::JumpIfZero(v1, false_label.clone()));

                    let v2 = self.emit_expression(right);

                    self.instructions
                        .push(Instruction::JumpIfZero(v2, false_label.clone()));

                    self.instructions
                        .push(Instruction::Copy(dst.clone(), Val::Constant(1)));
                    self.instructions.push(Instruction::Jump(end_label.clone()));

                    self.instructions.push(Instruction::Label(false_label));
                    self.instructions
                        .push(Instruction::Copy(dst.clone(), Val::Constant(0)));

                    self.instructions.push(Instruction::Label(end_label));

                    dst
                }

                ast::BinOp::LogicalOr => {
                    let dst = self.make_temporary();
                    let true_label = self.make_label("or_true");
                    let end_label = self.make_label("or_end");

                    let v1 = self.emit_expression(left);
                    self.instructions
                        .push(Instruction::JumpIfNotZero(v1, true_label.clone()));

                    let v2 = self.emit_expression(right);
                    self.instructions
                        .push(Instruction::JumpIfNotZero(v2, true_label.clone()));

                    self.instructions
                        .push(Instruction::Copy(dst.clone(), Val::Constant(0)));
                    self.instructions.push(Instruction::Jump(end_label.clone()));

                    self.instructions.push(Instruction::Label(true_label));
                    self.instructions
                        .push(Instruction::Copy(dst.clone(), Val::Constant(1)));

                    self.instructions.push(Instruction::Label(end_label));

                    dst
                }

                _ => {
                    let v1 = self.emit_expression(left);
                    let v2 = self.emit_expression(right);
                    let dst = self.make_temporary();

                    let tacky_op = match op {
                        ast::BinOp::Add => ir::BinaryOp::Add,
                        ast::BinOp::Subtract => ir::BinaryOp::Subtract,
                        ast::BinOp::Multiply => ir::BinaryOp::Multiply,
                        ast::BinOp::Divide => ir::BinaryOp::Divide,
                        ast::BinOp::Remainder => ir::BinaryOp::Remainder,
                        ast::BinOp::BitwiseAnd => ir::BinaryOp::BitwiseAnd,
                        ast::BinOp::BitwiseXor => ir::BinaryOp::BitwiseXor,
                        ast::BinOp::BitwiseOr => ir::BinaryOp::BitwiseOr,
                        ast::BinOp::LeftShift => ir::BinaryOp::LeftShift,
                        ast::BinOp::RightShift => ir::BinaryOp::RightShift,
                        ast::BinOp::Equal => ir::BinaryOp::Equal,
                        ast::BinOp::NotEqual => ir::BinaryOp::NotEqual,
                        ast::BinOp::LessThan => ir::BinaryOp::LessThan,
                        ast::BinOp::LessThanEqual => ir::BinaryOp::LessThanEqual,
                        ast::BinOp::GreaterThan => ir::BinaryOp::GreaterThan,
                        ast::BinOp::GreaterThanEqual => ir::BinaryOp::GreaterThanEqual,
                        ast::BinOp::LogicalAnd | ast::BinOp::LogicalOr => unreachable!(),
                    };

                    self.instructions
                        .push(Instruction::Binary(tacky_op, v1, v2, dst.clone()));
                    dst
                }
            },
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
