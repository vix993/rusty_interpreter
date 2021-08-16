use crate::types::{ByteCode, Program, ProgramError, Result, Variable};

macro_rules! make_op {
    ($code:expr, $op:tt) => {{
        if let Some(a) = $code.stack.pop() {
            if let Some(b) = $code.stack.pop() {
                $code.stack.push(Variable {
                    variable: None,
                    value: (b.value $op a.value),
                });
                None
            } else { Some(ProgramError::StackUnderflow) }
        } else { Some(ProgramError::StackUnderflow) }
    }
}}

pub fn interpret(bytecodes: Vec<ByteCode>) -> Result<Variable> {
    let mut code = Program {
        bytecodes,
        stack: Vec::new(),
    };

    for op in code.bytecodes {
        if let Some(err) = match op {
            ByteCode::LoadVal(i) => {
                code.stack.push(Variable {
                    variable: None,
                    value: i,
                });
                None
            }
            ByteCode::WriteVar(c) => {
                let loaded_value = code.stack.pop();
                if let Some(v) = loaded_value {
                    code.stack.push(Variable {
                        variable: Some(c),
                        value: v.value,
                    })
                }
                None
            }
            ByteCode::ReadVar(c) => {
                let read_value = code.stack.iter().find(|&&x| x.variable == Some(c));
                if let Some(v) = read_value {
                    let var = v.clone();
                    code.stack.push(Variable {
                        variable: var.variable,
                        value: var.value,
                    })
                }
                None
            }
            ByteCode::Mul => make_op!(code, *),
            ByteCode::Add => make_op!(code, +),
            ByteCode::Return => break,
        } {
            return Err(err);
        }
    }

    if let Some(v) = code.stack.pop() {
        Ok(v)
    } else {
        Err(ProgramError::StackUnderflow)
    }
}