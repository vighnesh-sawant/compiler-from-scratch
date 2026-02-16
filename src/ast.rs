#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(i32),
    UnaryOp((UnOp,Box<Expression>)),    
    BinaryOp(BinOp, Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    Negation,
    BitwiseComplement,
    LogicalNegation,
}


#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub body: Statement,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub function: FunctionDeclaration,
}
