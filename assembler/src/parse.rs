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
        _ => return Err(AssemblerError::InvalidOperationError(op.to_string())),
    };
    Ok(opcode)
}

/// Match a register number or name to its integer number.
pub fn match_register(reg: &str) -> Result<u8, AssemblerError> {
    if reg.starts_with('x') {
        match reg.strip_prefix('x').unwrap().parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(AssemblerError::NoSuchRegisterError(reg.to_string())),
        }
    } else {
        match REG_NAMES.iter().position(|e| *e == reg) {
            Some(n) => Ok(n as u8),
            None => Err(AssemblerError::NoSuchRegisterError(reg.to_string())),
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
                Err(AssemblerError::InvalidImmediateError(s.to_string()))
            }
        }
        Ok(d) => Ok(d as u32),
    }
}

/// Create a `Vec<String>` out of `&str`s.
macro_rules! to_owned_vec {
    ($($x:expr),+ $(,)?) => (
        vec!($($x.to_owned()),+)
    )
}

/// Helper function to break up an LI into an LUI and/or an ADDI.
fn generate_li(ir_tokens: &[String]) -> Result<Vec<Vec<String>>, AssemblerError> {
    let mut instructions: Vec<Vec<String>> = Vec::new();

    let immediate = parse_int!(i32, ir_tokens[2]);
    if immediate.is_err() {
        return Err(AssemblerError::InvalidImmediateError(
            ir_tokens[2].to_string(),
        ));
    }
    let immediate = immediate.unwrap();

    // Immediate fits in 12 bits.
    if (immediate as u32) <= 0xFFF {
        // We can just ADDI to x0.
        instructions.push(to_owned_vec![
            "addi",
            &ir_tokens[1],
            "x0",
            &format!("0x{:X}", immediate)
        ]);
    } else {
        let lower_twelve = immediate & 0x0000_0FFF;
        let upper_twenty = ((immediate as u32) & 0xFFFF_F000) >> 12;

        // Add upper bits by doing a LUI with the upper 22 bits.
        instructions.push(to_owned_vec![
            "lui",
            &ir_tokens[1],
            &format!("0x{:X}", upper_twenty)
        ]);
        if lower_twelve != 0 {
            // Add lower bits by doing an ADDI to x0 with the lower 10 bits.
            instructions.push(to_owned_vec![
                "addi",
                &ir_tokens[1],
                &ir_tokens[1],
                &format!("0x{:X}", lower_twelve)
            ]);
        }
    }

    Ok(instructions)
}

/// Take an instruction as tokens and return one or more vectors of tokens representing a base-instruction, or none
/// if it is not a psuedo-instruction. May raise an assembler error if an immediate needs to be parsed and fails.
pub fn transform_psuedo_ir(ir_tokens: &[String]) -> Result<Vec<Vec<String>>, AssemblerError> {
    let mut instructions: Vec<Vec<String>> = Vec::new();

    match &(*ir_tokens[0]) {
        "nop" => instructions.push(to_owned_vec!["addi", "x0", "x0", "0"]),
        "li" => {
            let transformed_ir = generate_li(ir_tokens);
            if let Err(why) = transformed_ir {
                return Err(why);
            }
            for t in transformed_ir.unwrap() {
                instructions.push(t);
            }
        }
        "mv" => instructions.push(to_owned_vec!["add", &ir_tokens[1], &ir_tokens[2], "x0"]),
        "not" => instructions.push(to_owned_vec!["xori", &ir_tokens[1], &ir_tokens[2], "-1"]),
        "neg" => instructions.push(to_owned_vec!["sub", &ir_tokens[1], "x0", &ir_tokens[2]]),
        "seqz" => instructions.push(to_owned_vec!["sltiu", &ir_tokens[1], &ir_tokens[2], "1"]),
        "snez" => instructions.push(to_owned_vec!["stlu", &ir_tokens[1], "x0", &ir_tokens[2]]),
        "sltz" => instructions.push(to_owned_vec!["slt", &ir_tokens[1], &ir_tokens[2], "x0"]),
        "sgtz" => instructions.push(to_owned_vec!["slt", &ir_tokens[1], "x0", &ir_tokens[2]]),
        "beqz" => instructions.push(to_owned_vec!["beq", &ir_tokens[1], "x0", &ir_tokens[2]]),
        "bnez" => instructions.push(to_owned_vec!["bne", &ir_tokens[1], "x0", &ir_tokens[2]]),
        "blez" => instructions.push(to_owned_vec!["bge", "x0", &ir_tokens[1], &ir_tokens[2]]),
        "bgez" => instructions.push(to_owned_vec!["bge", &ir_tokens[1], "x0", &ir_tokens[2]]),
        "bltz" => instructions.push(to_owned_vec!["blt", &ir_tokens[1], "x0", &ir_tokens[2]]),
        "bgtz" => instructions.push(to_owned_vec!["blt", "x0", &ir_tokens[1], &ir_tokens[2]]),
        "bgt" => instructions.push(to_owned_vec![
            "blt",
            &ir_tokens[2],
            &ir_tokens[1],
            &ir_tokens[3]
        ]),
        "ble" => instructions.push(to_owned_vec![
            "bge",
            &ir_tokens[2],
            &ir_tokens[1],
            &ir_tokens[3]
        ]),
        "j" => instructions.push(to_owned_vec!["jal", "x0", &ir_tokens[1]]),
        // TODO: support extended call
        "jal" | "call" => {
            if ir_tokens.len() == 2 {
                instructions.push(to_owned_vec!["jal", "ra", &ir_tokens[1]]);
            } else {
                instructions.push(ir_tokens.to_owned())
            }
        }
        "tail" => {
            instructions.push(to_owned_vec!["jal", "x0", &ir_tokens[1]]);
        }
        "jr" => instructions.push(to_owned_vec!["jalr", "x0", &ir_tokens[1], "0"]),
        "jalr" => {
            if ir_tokens.len() == 2 {
                instructions.push(to_owned_vec!["jalr", "ra", &ir_tokens[1], "0"]);
            } else {
                instructions.push(ir_tokens.to_owned())
            }
        }
        "ret" => instructions.push(to_owned_vec!["jalr", "x0", "ra", "0"]),
        _ => instructions.push(ir_tokens.to_owned()),
    }

    if !instructions.is_empty() {
        Ok(instructions)
    } else {
        Ok(vec![])
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
