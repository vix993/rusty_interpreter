use std::result;

#[derive(Copy, Clone)]
pub enum ByteCode {
    LoadVal(i64),
    WriteVar(char),
    ReadVar(char),
    Add,
    Mul,
    Return,
}

#[derive(Copy, Clone)]
pub struct Variable {
    variable: Option<char>,
    value: i64,
}

#[derive(Clone)]
pub struct Program {
    bytecodes: Vec<ByteCode>,
    stack: Vec<Variable>,
}

#[derive(Debug)]
pub enum ProgramError {
    StackUnderflow,
}

type Result<T> = result::Result<T, ProgramError>;

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

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    #[test]
    fn basic() {
        use ByteCode::*;
        let test_load_value_and_return = vec![LoadVal(2), Return];
        let test_load_two_values_multiply_return = vec![LoadVal(2), LoadVal(3), Mul, Return];
        let test_write_value = vec![LoadVal(2), WriteVar('c'), Return];
        let test_arithmetic_written_values = vec![
            LoadVal(1),
            WriteVar('x'),
            LoadVal(2),
            WriteVar('y'),
            ReadVar('x'),
            LoadVal(1),
            Add,
            ReadVar('y'),
            Mul,
            Return,
        ];

        let test_write_value_result = interpret(test_write_value).unwrap();

        assert_eq!(interpret(test_load_value_and_return).unwrap().value, 2);
        assert_eq!(
            interpret(test_load_two_values_multiply_return)
                .unwrap()
                .value,
            6
        );
        assert_eq!(test_write_value_result.variable, Some('c'));
        assert_eq!(test_write_value_result.value, 2);
        assert_eq!(interpret(test_arithmetic_written_values).unwrap().value, 4);
    }
}

fn main() {
    println!("Hello, world!");
}
