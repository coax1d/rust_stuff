use std::collections::HashMap;

type Offset = usize;

#[derive(Debug, Clone)]
pub enum Instruction {
    Load(i64),
    Read(String),
    Write(String),
    Jump(Offset),
    JumpIf(Offset),
    CompareEQ,
    CompareNE,
    CompareGT,
    CompareLT,
    CompareLTE,
    CompareGTE,
    Add,
    Sub,
    Mul,
    Div,
    Return,
}

pub struct ByteCode {
    code: Vec<Instruction>,
    stack: Vec<i64>,
    instruction_ptr: usize,
    vars: HashMap<String, i64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpreterError {
    UndefinedBehavior,
    DivideByZero,
    StackEmpty,
    BadInstructionOffset,
}

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

macro_rules! handleMath {
    {$byte_code:expr, $operator:tt} => {
        match $byte_code.stack.pop() {
            Some(rhs) => {
                match $byte_code.stack.pop() {
                    Some(lhs) => {
                        let result: i64 = (lhs $operator rhs) as i64;
                        $byte_code.stack.push(result);
                        Ok(())
                    },
                    _ => Err(InterpreterError::StackEmpty),
                }
            },
            _ => Err(InterpreterError::StackEmpty),
        }
}}

fn interpret(code: Vec<Instruction>) -> Result<i64, InterpreterError> {
    let mut byte_code = ByteCode {
        code: code,
        stack: Vec::new(),
        instruction_ptr: 0,
        vars: HashMap::new(),
    };

    loop {
        let instruction = &byte_code.code[byte_code.instruction_ptr];
        let op = match instruction {
            Instruction::Load(value) => {
                byte_code.stack.push(*value);
                Ok(())
            }
            Instruction::Write(var_name) => {
                match byte_code.stack.pop() {
                    Some(val) => {
                        byte_code.vars.insert(var_name.clone(), val);
                        Ok(())
                    }
                    _ => Err(InterpreterError::StackEmpty)
                }
            },
            Instruction::Read(var_name) => {
                match byte_code.vars.get(var_name) {
                    Some(read_val) => {
                        byte_code.stack.push(*read_val);
                        Ok(())
                    },
                    _ => Err(InterpreterError::UndefinedBehavior),
                }
            },
            Instruction::Add => handleMath!{byte_code, +},
            Instruction::Sub => handleMath!{byte_code, -},
            Instruction::Mul => handleMath!{byte_code, *},
            Instruction::Div => handleDiv!{byte_code},
            Instruction::CompareEQ => handleMath!{byte_code, ==},
            Instruction::CompareNE => handleMath!{byte_code, !=},
            Instruction::CompareGT => handleMath!{byte_code, >},
            Instruction::CompareLT => handleMath!{byte_code, <},
            Instruction::CompareGTE => handleMath!{byte_code, >=},
            Instruction::CompareLTE => handleMath!{byte_code, <=},
            Instruction::Jump(offset) => {
                if *offset >= byte_code.code.len() {
                    return Err(InterpreterError::BadInstructionOffset)
                }
                byte_code.instruction_ptr = *offset;
                Ok(())
            },
            Instruction::JumpIf(offset) => {
                match byte_code.stack.pop() {
                    Some(val) => {
                        if val == 0 {
                            byte_code.instruction_ptr = *offset;
                        }
                        Ok(())
                    },
                    _ => Err(InterpreterError::StackEmpty),
                }
            },
            Instruction::Return => break,
        };

        match op {
            Err(error_code) => return Err(error_code),
            Ok(()) => {},
        }

        byte_code.instruction_ptr += 1;
    }

    match byte_code.stack.pop() {
        Some(result) => Ok(result),
        _ => Err(InterpreterError::StackEmpty),
    }
}

#[cfg(test)]
mod tests {
    use super::{*, Instruction::*};

    #[test]
    fn load_val() {
        assert_eq!(interpret(vec![Load(1), Load(2), Load(-5), Return]).unwrap(), -5);
    }

    #[test]
    fn read_write_val() {
        assert_eq!(interpret(vec![Load(1), Write("x".into()), Load(5), Read("x".into()), Return]).unwrap(), 1);
    }

    #[test]
    fn add_val() {
        assert_eq!(interpret(vec![Load(1), Load(3), Add, Return]).unwrap(), 4);
        assert_eq!(interpret(vec![Load(3), Write("x".into()), Load(7),
            Write("y".into()), Read("x".into()), Read("y".into()), Add, Return]).unwrap(), 10);
    }

    #[test]
    fn sub_val() {
        assert_eq!(interpret(vec![Load(1), Load(3), Sub, Return]).unwrap(), -2);
    }

    #[test]
    fn mul_val() {
        assert_eq!(interpret(vec![Load(2), Load(3), Mul, Return]).unwrap(), 6);
    }

    #[test]
    fn div_val() {
        assert_eq!(interpret(vec![Load(4), Load(2), Div, Return]).unwrap(), 2);
    }

    #[test]
    fn div_by_zero() {
        assert!(interpret(vec![Load(2), Load(0), Div, Return]).is_err());
    }

    #[test]
    fn test_from_assignment() {
        let assignment_byte_code = vec![Load(1), Write("x".into()), Load(3),
            Write("y".into()), Read("x".into()), Load(1), Add, Read("y".into()), Mul, Return];
        assert_eq!(interpret(assignment_byte_code).unwrap(), 6);
    }

    #[test]
    fn test_unconditional_jump() {
        assert_eq!(interpret(vec![Load(4), Jump(2), Load(5), Load(7), Add, Return]).unwrap(), 11);
    }

    #[test]
    fn test_lt_loop() {
        /*
         i = 0
         while i < 3
            i += 1
         done
         */
        assert_eq!(interpret(vec![Load(0), Write("i".into()), Read("i".into()), Load(3),
            CompareLT, JumpIf(10), Read("i".into()), Load(1), Add, Write("i".into()),
            Jump(1), Read("i".into()), Return]).unwrap(), 3);
    }

    // Further tests for each conditional...
}
