use crate::{
    constants::*, encode_b_imm, encode_func3, encode_func7, encode_i_imm, encode_j_imm, encode_rd,
    encode_rs1, encode_rs2, encode_s_imm, encode_u_imm, parse_int, AssemblerError,
};
use std::{collections::HashMap, io::prelude::*};

enum InstructionFormat {
    Itype,
    Rtype,
    Jtype,
    Utype,
    Stype,
    Btype,
}

/// Convert an instruction to it's tokens.
/// Immediates always come last.
/// Undefined behavior for malformed instructions.
fn tokenize(s: &str) -> Vec<String> {
    let mut in_token = false;
    let mut in_parens = false;
    let mut token_start = 0;
    let mut num_tokens = 0;

    let mut tokens = vec!["".to_string(); 4];

    for (i, c) in s.chars().enumerate() {
        match c {
            ' ' | ',' => {
                if in_token {
                    in_token = false;
                    // Shift the token over one if its surround by parentheses (for offsets).
                    if !in_parens {
                        tokens[num_tokens] = s[token_start..i].to_string();
                        num_tokens += 1;
                    }
                }
            }
            '(' => {
                in_token = false;
                in_parens = true;
                tokens[num_tokens + 1] = s[token_start..i].to_string();
            }
            ')' => {
                tokens[num_tokens] = s[token_start..i].to_string();
            }
            _ => {
                if !in_token {
                    in_token = true;
                    token_start = i;
                }
            }
        }
    }

    if !in_parens {
        tokens[num_tokens] = s[token_start..].to_string();
    }

    tokens
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

    // Use the opcode to identify the instruction format.
    let format = match opcode {
        OPCODE_ARITHMETIC_IMM | OPCODE_JALR | OPCODE_LOAD => InstructionFormat::Itype,
        OPCODE_ARITHMETIC => InstructionFormat::Rtype,
        OPCODE_JAL => InstructionFormat::Jtype,
        OPCODE_LUI | OPCODE_AUIPC => InstructionFormat::Utype,
        OPCODE_BRANCH => InstructionFormat::Btype,
        OPCODE_STORE => InstructionFormat::Stype,
        _ => unreachable!(),
    };

    // Use the destination register field.
    if let InstructionFormat::Rtype | InstructionFormat::Itype | InstructionFormat::Utype = format {
        let rd = match_register(&tokens[1]);
        if let Err(why) = rd {
            return Err(why);
        }
        ir |= encode_rd!(rd.unwrap());
    }

    // Use the first register operand and func3 fields.
    if let InstructionFormat::Itype
    | InstructionFormat::Rtype
    | InstructionFormat::Btype
    | InstructionFormat::Stype = format
    {
        let rs1 = match_register(&tokens[2]);
        if let Err(why) = rs1 {
            return Err(why);
        }
        ir |= encode_rs1!(rs1.unwrap());

        let func3 = match op {
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
            _ => unreachable!(),
        };
        ir |= encode_func3!(func3);
    }

    // Use the second register operand field.
    if let InstructionFormat::Rtype | InstructionFormat::Stype | InstructionFormat::Btype = format {
        let rs2 = match_register(&tokens[2]);
        if let Err(why) = rs2 {
            return Err(why);
        }
        ir |= encode_rs2!(rs2.unwrap());
    }

    // Use the func7 field.
    if let InstructionFormat::Rtype = format {
        let func7 = match op {
            "add" | "addi" => FUNC7_ADD,
            "sub" => FUNC7_SUB,
            "sra" | "srai" => FUNC7_SRA,
            "srl" | "srli" => FUNC7_SRL,
            _ => unreachable!(),
        };
        ir |= encode_func7!(func7);
    }

    match format {
        InstructionFormat::Itype => {
            let imm = parse_int!(i32, &tokens[3]);
            if imm.is_err() {
                return Err(AssemblerError::InvalidImmediateError);
            }
            let imm = imm.unwrap();
            ir |= encode_i_imm!(imm);
        }
        InstructionFormat::Utype => {
            let imm = parse_int!(u32, &tokens[2]);
            if imm.is_err() {
                return Err(AssemblerError::InvalidImmediateError);
            }
            let imm = imm.unwrap();
            ir |= encode_u_imm!(imm);
        }
        InstructionFormat::Jtype => {
            let imm = parse_int!(u32, &tokens[2]);
            if imm.is_err() {
                return Err(AssemblerError::InvalidImmediateError);
            }
            let imm = imm.unwrap();
            ir |= encode_j_imm!(imm);
        }
        InstructionFormat::Btype => {
            let imm = parse_int!(i32, &tokens[3]);
            if imm.is_err() {
                return Err(AssemblerError::InvalidImmediateError);
            }
            let imm = imm.unwrap();
            ir |= encode_b_imm!(imm);
        }
        InstructionFormat::Stype => {
            let imm = parse_int!(i32, &tokens[3]);
            if imm.is_err() {
                return Err(AssemblerError::InvalidImmediateError);
            }
            let imm = imm.unwrap();
            ir |= encode_s_imm!(imm);
        }
        _ => unreachable!(),
    }

    Ok(ir)
}

pub fn assemble_stream<R>(reader: R) -> Vec<u32>
where
    R: Read,
{
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            vec![
                "addi".to_string(),
                "t0".to_string(),
                "t1".to_string(),
                "17".to_string()
            ],
            tokenize("addi t0, t1, 17")
        );
    }

    #[test]
    fn test_tokenize_with_offsets() {
        assert_eq!(
            vec![
                "lw".to_string(),
                "t0".to_string(),
                "s0".to_string(),
                "17".to_string()
            ],
            tokenize("lw t0, 17(s0)")
        );
    }
}
