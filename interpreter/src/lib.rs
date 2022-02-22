use std::collections::HashMap;

#[derive(Debug)]
pub enum OpCode {
    Load(i64),
    Read(String),
    Write(String),
    Add,
    Sub,
    Mul,
    Div,
    Return,
}
pub struct ByteCode {
    code: Vec<OpCode>,
    stack: Vec<i64>,
    vars: HashMap<String, i64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpreterError {
    UndefinedBehavior,
    DivideByZero,
    StackEmpty,
}

macro_rules! handleMath {
    {$byte_code:expr, $operator:tt} => {
        match $byte_code.stack.pop() {
            Some(rhs) => {
                match $byte_code.stack.pop() {
                    Some(lhs) => {
                        $byte_code.stack.push(lhs $operator rhs);
                        Ok(())
                    },
                    _ => Err(InterpreterError::StackEmpty),
                }
            },
            _ => Err(InterpreterError::StackEmpty),
        }
}}

macro_rules! handleDiv {
    {$byte_code:expr} => {
    match $byte_code.stack.pop() {
        Some(rhs) => {
            match $byte_code.stack.pop() {
                Some(lhs) => {
                    if rhs == 0 {
                        return Err(InterpreterError::DivideByZero)
                    }
                    else {
                        $byte_code.stack.push(lhs / rhs);
                        Ok(())
                    }
                },
                _ => Err(InterpreterError::StackEmpty)
            }
        },
        _ => Err(InterpreterError::StackEmpty)
    }
}}

fn interpret(code: Vec<OpCode>) -> Result<i64, InterpreterError> {
    let mut byte_code = ByteCode {
        code: code,
        stack: Vec::new(),
        vars: HashMap::new(),
    };

    for instruction in byte_code.code {
        let op = match instruction {
            OpCode::Load(value) => {
                byte_code.stack.push(value);
                Ok(())
            }
            OpCode::Write(var_name) => {
                match byte_code.stack.pop() {
                    Some(val) => {
                        byte_code.vars.insert(var_name, val);
                        Ok(())
                    }
                    _ => Err(InterpreterError::StackEmpty)
                }
            },
            OpCode::Read(var_name) => {
                match byte_code.vars.get(&var_name) {
                    Some(read_val) => {
                        byte_code.stack.push(*read_val);
                        Ok(())
                    },
                    _ => Err(InterpreterError::UndefinedBehavior),
                }
            },
            OpCode::Add => handleMath!{byte_code, +},
            OpCode::Sub => handleMath!{byte_code, -},
            OpCode::Mul => handleMath!{byte_code, *},
            OpCode::Div => handleDiv!{byte_code},
            OpCode::Return => break,
        };

        match op {
            Err(error_code) => return Err(error_code),
            Ok(()) => {},
        }
    }

    match byte_code.stack.pop() {
        Some(result) => Ok(result),
        _ => Err(InterpreterError::StackEmpty),
    }
}

#[cfg(test)]
mod tests {
    use super::{*, OpCode::*};

    #[test]
    fn load_val() {
        assert_eq!(interpret(vec![Load(1), Load(2), Load(-5)]).unwrap(), -5);
    }

    #[test]
    fn read_write_val() {
        assert_eq!(interpret(vec![Load(1), Write("x".into()), Load(5), Read("x".into())]).unwrap(), 1);
    }

    #[test]
    fn add_val() {
        assert_eq!(interpret(vec![Load(1), Load(3), Add]).unwrap(), 4);
        assert_eq!(interpret(vec![Load(3), Write("x".into()), Load(7),
            Write("y".into()), Read("x".into()), Read("y".into()), Add]).unwrap(), 10);
    }

    #[test]
    fn sub_val() {
        assert_eq!(interpret(vec![Load(1), Load(3), Sub]).unwrap(), -2);
    }

    #[test]
    fn mul_val() {
        assert_eq!(interpret(vec![Load(2), Load(3), Mul]).unwrap(), 6);
    }

    #[test]
    fn div_val() {
        assert_eq!(interpret(vec![Load(4), Load(2), Div]).unwrap(), 2);
    }

    #[test]
    fn div_by_zero() {
        assert!(interpret(vec![Load(2), Load(0), Div]).is_err());
    }

    #[test]
    fn test_from_assignment() {
        let assignment_byte_code = vec![Load(1), Write("x".into()), Load(3),
            Write("y".into()), Read("x".into()), Load(1), Add, Read("y".into()), Mul, Return];
        assert_eq!(interpret(assignment_byte_code).unwrap(), 6);
    }
}
