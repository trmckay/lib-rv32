use std::collections::HashMap;

use lib_rv32_common::{constants::*, parse_int};

use crate::error::AssemblerError;

/// Convert an instruction to it's tokens, stripping out whitespace,
/// parenthesis, and commas.
#[macro_export]
macro_rules! tokenize {
    ($s:expr) => {
        $s.replace(",", " ")
            .replace("\n", " ")
            .replace("(", " ")
            .replace(")", " ")
            .to_ascii_lowercase()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();
    };
}

/// Match an operation to the correct opcode.
pub fn match_opcode(op: &str) -> Result<u8, AssemblerError> {
    let opcode = match op {
        "add" | "sub" | "sll" | "slt" | "sltu" | "xor" | "sra" | "or" | "and" => OPCODE_ARITHMETIC,
        "addi" | "slli" | "slti" | "xori" | "srai" | "ori" | "andi" => OPCODE_ARITHMETIC_IMM,
        "lui" => OPCODE_LUI,
        "auipc" => OPCODE_AUIPC,
        "jal" => OPCODE_JAL,
        "jalr" => OPCODE_JALR,
        "beq" | "bne" | "blt" | "bge" | "bgeu" => OPCODE_BRANCH,
        "lb" | "lbu" | "lh" | "lhu" | "lw" => OPCODE_LOAD,
        "sb" | "sh" | "sw" => OPCODE_STORE,
        _ => return Err(AssemblerError::InvalidOperationError),
    };
    Ok(opcode)
}

/// Match a register number or name to its integer number.
pub fn match_register(reg: &str) -> Result<u8, AssemblerError> {
    if reg.starts_with('x') {
        match reg.strip_prefix('x').unwrap().parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(AssemblerError::NoSuchRegisterError),
        }
    } else {
        match REG_NAMES.iter().position(|e| *e == reg) {
            Some(n) => Ok(n as u8),
            None => Err(AssemblerError::NoSuchRegisterError),
        }
    }
}

/// Parse a label or an immediate literal into an integer.
pub fn parse_imm(s: &str, labels: &HashMap<String, u32>, pc: u32) -> Result<u32, AssemblerError> {
    let num = parse_int!(i64, s);
    match num {
        Err(_) => {
            let label = labels.get(s);
            if let Some(v) = label {
                Ok((*v).wrapping_sub(pc))
            } else {
                Err(AssemblerError::InvalidImmediateError)
            }
        }
        Ok(d) => Ok(d as u32),
    }
}

/// Match an operation to the correct func3.
#[macro_export]
macro_rules! match_func3 {
    ($t:expr) => {
        match $t {
            "beq" => FUNC3_BEQ,
            "bne" => FUNC3_BNE,
            "blt" => FUNC3_BLT,
            "bge" => FUNC3_BGE,
            "bltu" => FUNC3_BLTU,
            "bgeu" => FUNC3_BGEU,
            "lb" => FUNC3_LB,
            "lbu" => FUNC3_LBU,
            "lh" => FUNC3_LH,
            "lhu" => FUNC3_LHU,
            "lw" => FUNC3_LW,
            "sb" => FUNC3_SB,
            "sh" => FUNC3_SH,
            "sw" => FUNC3_SW,
            "add" | "addi" | "sub" => FUNC3_ADD_SUB,
            "sll" | "slli" => FUNC3_SLL,
            "slt" | "slti" => FUNC3_SLT,
            "sltu" => FUNC3_SLTU,
            "xor" | "xori" => FUNC3_XOR,
            "sra" | "srai" | "srl" | "srli" => FUNC3_SR,
            "or" | "ori" => FUNC3_OR,
            "and" | "andi" => FUNC3_AND,
            _ => 0,
        }
    };
}

/// Match an operation to the correct func7.
#[macro_export]
macro_rules! match_func7 {
    ($t:expr) => {
        match $t {
            "add" | "addi" => FUNC7_ADD,
            "sub" => FUNC7_SUB,
            "sra" | "srai" => FUNC7_SRA,
            "srl" | "srli" => FUNC7_SRL,
            _ => unreachable!(),
        }
    };
}
