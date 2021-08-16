use std::result;

#[derive(Copy, Clone)]
pub enum ByteCode {
    LoadVal(i64),
    WriteVar(char),
    ReadVar(char),
    Add,
    Mul,
	Div,
	Sub,
    Return,
}

#[derive(Copy, Clone, Debug)]
pub struct Variable {
    pub variable: Option<char>,
    pub value: i64,
}

#[derive(Clone)]
pub struct Program {
    pub bytecodes: Vec<ByteCode>,
    pub stack: Vec<Variable>,
}

#[derive(Debug)]
pub enum ProgramError {
    StackUnderflow,
}

pub type Result<T> = result::Result<T, ProgramError>;