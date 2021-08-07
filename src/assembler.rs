use crate::{constants::*, error::AssemblerError};
use crate::{encode_i_imm, encode_rd, encode_rs1};
use std::collections::HashMap;
use std::io::prelude::*;

enum InstructionFormat {
    Itype,
    Rtype,
    Jtype,
    Utype,
    Stype,
    Btype,
}

fn tokenize(s: &str) -> Vec<String> {
    let s = s.to_owned().to_lowercase().replace(",", "");
    let v: Vec<String> = s.split(' ').map(|s| s.replace(" ", "")).collect();
    v
}

fn match_opcode(op: &str) -> Result<u8, AssemblerError> {
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

fn match_register(reg: &str) -> Result<u8, AssemblerError> {
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

pub fn assemble_ir(ir_string: &str, labels: &HashMap<String, u32>) -> Result<u32, AssemblerError> {
    let mut ir: u32 = 0;

    let tokens = tokenize(ir_string);
    if tokens.is_empty() {
        return Err(AssemblerError::TooFewTokensError);
    } else if tokens.len() > 4 {
        return Err(AssemblerError::TooManyTokensError);
    }

    let op = &tokens[0][..];
    let opcode = match_opcode(op);
    if let Err(why) = opcode {
        return Err(why);
    }
    let opcode = opcode.unwrap();
    ir |= (opcode & 0b1111111) as u32;

    let format = match opcode {
        // I-type
        OPCODE_ARITHMETIC_IMM | OPCODE_JALR | OPCODE_LOAD => InstructionFormat::Itype,
        OPCODE_ARITHMETIC => InstructionFormat::Rtype,
        OPCODE_JAL => InstructionFormat::Jtype,
        OPCODE_LUI | OPCODE_AUIPC => InstructionFormat::Utype,
        OPCODE_BRANCH => InstructionFormat::Btype,
        OPCODE_STORE => InstructionFormat::Stype,
        _ => unreachable!(),
    };

    match format {
        InstructionFormat::Itype => {
            let rd = match_register(&tokens[1]);
            if let Err(why) = rd {
                return Err(why);
            }
            ir |= encode_rd!(rd.unwrap());

            let rs1 = match_register(&tokens[2]);
            if let Err(why) = rs1 {
                return Err(why);
            }
            ir |= encode_rs1!(rs1.unwrap());

            let imm: Result<i16, _> = tokens[3].parse();
            if imm.is_err() {
                return Err(AssemblerError::InvalidImmediateError);
            }
            let imm = imm.unwrap();
            if imm < -2048 || imm > 2047 {
                return Err(AssemblerError::ImmediateTooLargeError);
            }
            ir |= encode_i_imm!(imm);
        }

        InstructionFormat::Btype => {}

        InstructionFormat::Jtype => {}

        InstructionFormat::Rtype => {}

        InstructionFormat::Stype => {}

        InstructionFormat::Utype => {}
    };

    Ok(ir)
}

pub fn assemble_stream<R>(reader: R) -> Vec<u32>
where
    R: Read,
{
    vec![]
}
