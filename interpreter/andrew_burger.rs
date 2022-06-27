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

// (4) Write a function that given a directory, recursively finds all files with a given file
//     extension in that directory and all sub-directories, and counts the number of lines
//     in the file and prints it to stdout.

// A file system is in a tree structure so Depth First Search (Recursive approach) which
// traverse the tree to the furthest leaf node(i.e. the furthest down directory/file)
// This is O(n) runtime (where n is a leaf in the tree).
// After recursing every directory and file match on the particular file extension
// open the file and read the lines O(Lines) runtime complexity
// Bringin the total complexity of the algorithm to O(N * L) (Worst case everything is
// a leaf node) for each leaf which matches your file extension you read All of its lines.

// Not sure if it is ok to use these crates but runnin out of time.
use std::fs::File;
use std::io::{BufRead, BufReader};
use glob::{glob};
fn find_all_files(file_path: &str, extension: &str) -> Result<(), std::io::Error> {
    // This tells glob to recurse all sub dirs and to grab all file extensions in those subdirs
    const RECURSIVE_GLOB_MAGIC: &str = "/**/*.";
    let glob_string = format!("{}{}{}", file_path, RECURSIVE_GLOB_MAGIC, extension);
    for entry in glob(glob_string.as_str()).expect("Incorrect Glob string")
      {
        let mut line_count = 0;
        let file_path = entry.expect("Incorrect PathBuf");
        println!("file {}", file_path.to_str().unwrap());
        let file = BufReader::new(File::open(file_path)?);
        for _ in file.lines() {
          line_count += 1;
        }
        println!("Total lines are: {}", line_count);
  }
    Ok(())
}

// (5) explain some of the ways hashing functions enable blockchain technology

// Allows the encoding of data. It is helpful to have a representation of some piece of data which can be
// public yet the contents of the data  Be private. Public Private key cryptography allows for the trustless
// verification of signatures. This is extremely useful because it allows for A decentralized method for
// verification, where both parties can agree on a source of truth, trusting the power of a math proof
// instead of each other.

// (6) briefly explain Bitcoin's UTXO model of transaction validation (separate from POW)

// As opposed to an Accounts model the UTXO model adds a layer of privacy. UTXOS are a form of change
// following a transaction. i.e. Alice sends 1 bitcoin but only has UTXO denominations of .6 and .5
// meaning Alice will have an unspent output of the transaction Equalling .1 BTC. This will be her
// change and will not technically be "Deposited" into her account but will be stored under a specific
// address on chain. A users BTC Account is a value but it will be broken into a series of UTXO hashs.

// (7) what is the structure of a Block in bitcoin and how does it relate to the 'blockchain'
// (merkle tree vs merkle list of merkle trees)

// Each bitcoin block contains several fields one of which being a merkle root. A merkle root
// (The root of a merkle tree) is a summation of the hashes of its children. The leaf nodes are
// transaction hashes. The merkle root allows for easy verification of the transactions which occurred
// in a single block. Since block data is shared over a p2p network (Where information is gossiped i.e.
// data is spread to peers in various chunks), the merkle tree allows for a very nice elegant verification
// of a block, where bad actors who may attempt to alter the transactions of a particular tree.
// The blockchain is in simple terms the concatenation of all of these merkle roots(transaction data)
// i.e. a merkle list. There is other pieces of a bitcoin block such as a short script
// (which is not Turing complete i.e. loops) and block number etc.


// (8) what problem/s are POW/POS trying to solve? discuss/compare
// (byzantine fault tolerance, reaching a single consensus on a p2p network)

// Pow/Pos are consensus algorithms for agreeing on the next state of a block chain(i.e. the next block).
// This comes from the Byzantine Generals problem which is somewhat analogous to the various forks which
// can occur from nodes in a p2p network. Blockchains must be Byzantine fault tolerant, i.e. there may be
// seemingly good actors in a distributed network but they are indeed malicious. By implementing a PoW/PoS
// system the majority of honest nodes in a network can find agreement on the next state of the blockchain.
// PoS in particular can attempt to isolate bad actors and punishing them further by slashing their stake and
// reputation.