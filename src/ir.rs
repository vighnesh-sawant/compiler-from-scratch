#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negation,
    BitwiseComplement,
    LogicalNegation,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

#[derive(Debug, Clone)]
pub enum Val {
    Constant(i32),
    Var(String), // Represents both variable names ("x") and temporaries ("tmp.0")
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Val),
    Unary(UnaryOp, Val, Val),       // op src, dst
    Binary(BinaryOp, Val, Val, Val), // op src1, src2, dst
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
