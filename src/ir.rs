#[derive(Debug, Clone, PartialEq)]
pub enum Reg {
    RAX,
    RCX, // We only need these two for now based on your code
    AL, // Lower 8 bits of AX (needed for 'sete')
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Imm(i32),
    Reg(Reg),
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
