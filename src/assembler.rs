use crate::{
    constants::*, encode_b_imm, encode_func3, encode_func7, encode_i_imm, encode_j_imm,
    encode_opcode, encode_rd, encode_rs1, encode_rs2, encode_s_imm, encode_u_imm, parse_int,
    AssemblerError,
};
use std::{collections::HashMap, io::prelude::*};

use log::info;

enum InstructionFormat {
    Itype,
    Rtype,
    Jtype,
    Utype,
    Stype,
    Btype,
}

/// Convert an instruction to it's tokens, stripping out whitespace,
/// parenthesis, and commas.
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

fn parse_imm(s: &str, labels: &HashMap<String, u32>, pc: u32) -> Result<u32, AssemblerError> {
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
            _ => unreachable!(),
        }
    };
}

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

pub fn assemble_ir(
    ir_string: &str,
    labels: &mut HashMap<String, u32>,
    pc: u32,
) -> Result<u32, AssemblerError> {
    let mut ir: u32 = 0;

    info!("'{}'", ir_string);

    let mut tokens: Vec<String> = tokenize!(ir_string);

    info!(" -> {:?}", tokens);

    if tokens.is_empty() {
        return Err(AssemblerError::TooFewTokensError);
    } else if tokens.len() > 5 {
        return Err(AssemblerError::TooManyTokensError);
    }

    // Add and remove leading label.
    if tokens[0].ends_with(':') {
        labels.insert(tokens[0].strip_suffix(':').unwrap().to_owned(), pc);
        tokens.remove(0);
    }

    let op = &tokens[0][..];
    let opcode = match_opcode(op);
    if let Err(why) = opcode {
        return Err(why);
    }
    let opcode = opcode.unwrap();
    ir |= encode_opcode!(opcode);

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
        let rs1 = match_register(
            &tokens[match opcode {
                OPCODE_LOAD => 3,
                _ => 2,
            }],
        );
        if let Err(why) = rs1 {
            return Err(why);
        }
        ir |= encode_rs1!(rs1.unwrap());

        ir |= encode_func3!(match_func3!(op));
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
        ir |= encode_func7!(match_func7!(op));
    }

    match format {
        InstructionFormat::Itype => {
            let imm = parse_imm(
                &tokens[match opcode {
                    OPCODE_LOAD => 2,
                    _ => 3,
                }],
                labels,
                pc,
            );
            if imm.is_err() {
                return imm;
            }
            let imm = imm.unwrap();
            ir |= encode_i_imm!(imm);
        }
        InstructionFormat::Utype => {
            let imm = parse_imm(&tokens[2], labels, pc);
            if imm.is_err() {
                return imm;
            }
            let imm = imm.unwrap();
            ir |= encode_u_imm!(imm);
        }
        InstructionFormat::Jtype => {
            let imm = parse_imm(&tokens[2], labels, pc);
            if imm.is_err() {
                return imm;
            }
            let imm = imm.unwrap();
            ir |= encode_j_imm!(imm);
        }
        InstructionFormat::Btype => {
            let imm = parse_imm(&tokens[3], labels, pc);
            if imm.is_err() {
                return imm;
            }
            let imm = imm.unwrap();
            ir |= encode_b_imm!(imm);
        }
        InstructionFormat::Stype => {
            let imm = parse_imm(&tokens[2], labels, pc);
            if imm.is_err() {
                return imm;
            }
            let imm = imm.unwrap();
            ir |= encode_s_imm!(imm);
        }
        InstructionFormat::Rtype => (),
    }

    info!(" -> {:08x}\n", ir);
    Ok(ir)
}

pub fn assemble_buf<R>(reader: &mut R) -> Result<Vec<u32>, AssemblerError>
where
    R: BufRead,
{
    let mut prog = Vec::new();
    let mut buf = String::new();
    let mut labels = HashMap::new();
    let mut pc: u32 = 0;

    loop {
        let bytes_rd = reader.read_line(&mut buf);

        if bytes_rd.is_err() {
            return Err(AssemblerError::IOError);
        }

        if bytes_rd.unwrap() == 0 {
            break;
        }

        let ir = assemble_ir(buf.trim_end(), &mut labels, pc);

        if let Err(why) = ir {
            return Err(why);
        }

        prog.push(ir.unwrap());
        buf.clear();
        pc += 4;
    }

    Ok(prog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_tokenize() {
        let tokens: Vec<String> = tokenize!("addi t0, t1, 17");
        assert_eq!(
            vec![
                "addi".to_string(),
                "t0".to_string(),
                "t1".to_string(),
                "17".to_string()
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_with_offsets() {
        let tokens: Vec<String> = tokenize!("lw t0, 17(s0)");
        assert_eq!(
            vec![
                "lw".to_string(),
                "t0".to_string(),
                "17".to_string(),
                "s0".to_string(),
            ],
            tokens
        );
        let tokens: Vec<String> = tokenize!("lw x5, 0(x5)");
        assert_eq!(
            vec![
                "lw".to_string(),
                "x5".to_string(),
                "0".to_string(),
                "x5".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_many_commas() {
        let tokens: Vec<String> = tokenize!("lw,,, t0,,,,, 17,,,(s0),,,,,,");
        assert_eq!(
            vec![
                "lw".to_string(),
                "t0".to_string(),
                "17".to_string(),
                "s0".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_many_spaces() {
        let tokens: Vec<String> = tokenize!("lw    t0      17   (s0)      ");
        assert_eq!(
            vec![
                "lw".to_string(),
                "t0".to_string(),
                "17".to_string(),
                "s0".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_tokenize_label() {
        let tokens: Vec<String> = tokenize!("label: addi t0, t1, 12");
        assert_eq!(
            vec![
                "label:".to_string(),
                "addi".to_string(),
                "t0".to_string(),
                "t1".to_string(),
                "12".to_string(),
            ],
            tokens
        );
    }

    #[test]
    fn test_parse_imm() {
        let mut labels: HashMap<String, u32> = HashMap::new();
        labels.insert("loop".to_string(), 0);
        let pc = 4;

        assert_eq!(-4, parse_imm("loop", &labels, pc).unwrap() as i32);
        assert_eq!(-24, parse_imm("-24", &labels, pc).unwrap() as i32);
        assert_eq!(16, parse_imm("16", &labels, pc).unwrap());
    }
}
