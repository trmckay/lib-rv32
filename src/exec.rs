use crate::decode::*;
use crate::{b_imm, func3, func7, i_imm, j_imm, opcode, rd, rs1, rs2, s_imm, u_imm};
use crate::{traits::Memory, traits::RegisterFile, RiscvError, REG_NAMES};

use log::info;

// NOTE on signedness:
//
// We interpret everything as an unsigned 32-bit integer (u32).
// This works great since the assembler encodes signed numbers
// as two's-complement. So, if we read '-1' encoded in 2C from the
// register-file, then add it to another number, the overflow
// will result in a correct operation. There is--in general--no
// need to care about the signedness of what we are operating on.
//
// The only exception to this is comparisons and loading
// bytes/half-words. For comparisons, we need to consider the sign
// due to the sign bit being the MSB. For loading units smaller than
// a word, we need to sign extend them before putting them into the
// 32-bit registers.

// NOTE on testing:
// I think it makes the most sense to test this is the integration tests,
// since it requires a working memory and register file. See
// tests/integration.rs for these tests.

/// Decode and execute instruction provided as a byte-slice.
///
/// Parameters:
///     `ir`: The instruction
///     `mem`: The memory which implements the `Memory` trait
///     `rf`: The register file which implements the `RegisterFile` trait
///
/// Returns:
///     `Result<u32, RiscvError>`: Returns the next program counter and
///     the error that occurred during execution, if one exists.
pub fn exec_one<M, R>(pc: &mut u32, mem: &mut M, rf: &mut R) -> Result<(), RiscvError>
where
    M: Memory,
    R: RegisterFile,
{
    let ir = mem.fetch(*pc);
    if let Err(why) = ir {
        return Err(why);
    }
    let ir = ir.unwrap();
    let opcode = opcode!(ir) as u8;

    info!("[{:04x}]  {:08x}  |  ", pc, ir);

    return match opcode {
        OPCODE_LUI => {
            let rd = rd!(ir);
            let imm = u_imm!(ir);

            info!(
                "{:25} |  ",
                format!(
                    "{:6} {}, 0x{:x} ({})",
                    "lui", REG_NAMES[rd as usize], imm, imm as i32
                )
            );

            if let Err(why) = rf.write(rd, imm) {
                return Err(why);
            }
            *pc += 4;

            info!("\n");
            Ok(())
        }

        OPCODE_AUIPC => {
            let rd = rd!(ir);
            let imm = u_imm!(ir);

            info!(
                "{:25} |  ",
                format!(
                    "{:6} {}, 0x{:x}",
                    "auipc",
                    REG_NAMES[rd as usize],
                    (imm >> 12)
                )
            );

            if let Err(why) = rf.write(rd, *pc + imm) {
                return Err(why);
            }
            *pc += 4;

            info!("\n");
            Ok(())
        }

        OPCODE_JAL => {
            let rd = rd!(ir);
            let imm = j_imm!(ir);

            info!(
                "{:25} |  ",
                format!(
                    "{:6} {}, 0x{:x} ({})",
                    "jal", REG_NAMES[rd as usize], imm, imm as i32
                )
            );

            if let Err(why) = rf.write(rd, *pc + 4) {
                return Err(why);
            }
            *pc = pc.overflowing_add(imm).0;
            info!("pc <- 0x{:x}; ", pc);

            info!("\n");
            Ok(())
        }

        OPCODE_JALR => {
            let rd = rd!(ir);
            let rs1 = rs1!(ir);
            let imm = i_imm!(ir);

            info!(
                "{:25} |  ",
                format!(
                    "{:6} {}, ({}){}",
                    "jalr", REG_NAMES[rd as usize], imm as i32, REG_NAMES[rs1 as usize]
                )
            );

            let rs1_data = match rf.read(rs1) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            if let Err(why) = rf.write(rd, *pc + 4) {
                return Err(why);
            }

            *pc = rs1_data.overflowing_add(imm).0;
            info!("pc <- 0x{:x}; ", pc);

            info!("\n");
            Ok(())
        }

        OPCODE_BRANCH => {
            let rs1 = rs1!(ir);
            let rs1_data = match rf.read(rs1) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            let rs2 = rs2!(ir);
            let rs2_data = match rf.read(rs2) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            let func3 = func3!(ir);
            let taken = match func3 {
                FUNC3_BEQ => rs1_data == rs2_data,
                FUNC3_BNE => rs1_data != rs2_data,
                FUNC3_BLT => rs1_data < rs2_data,
                FUNC3_BGE => rs1_data >= rs2_data,
                FUNC3_BLTU => rs1_data < rs2_data,
                FUNC3_BGEU => rs1_data > rs2_data,
                _ => return Err(RiscvError::InvalidFunc3Error(ir, func3)),
            };
            let imm = b_imm!(ir);

            info!(
                "{:25} |  {}",
                format!(
                    "{:6} {}, {}, {}",
                    match func3 {
                        FUNC3_BEQ => "beq",
                        FUNC3_BNE => "bne",
                        FUNC3_BLT => "blt",
                        FUNC3_BGE => "bge",
                        FUNC3_BLTU => "bltu",
                        FUNC3_BGEU => "bgeu",
                        _ => "",
                    },
                    REG_NAMES[rs1 as usize],
                    REG_NAMES[rs2 as usize],
                    imm as i32,
                ),
                if taken {
                    "branch taken; "
                } else {
                    "branch not taken; "
                }
            );

            if taken {
                *pc = pc.overflowing_add(imm).0;
                info!("pc <- 0x{:x}; ", pc);
            } else {
                *pc += 4;
            }

            info!("\n");
            Ok(())
        }

        OPCODE_LOAD => {
            let rs1 = rs1!(ir);
            let base = match rf.read(rs1) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            let imm = i_imm!(ir);
            let addr = base.overflowing_add(imm).0;
            let rd = rd!(ir);
            let func3 = func3!(ir);

            info!(
                "{:25} |  ",
                format!(
                    "{:6} {}, {}({})",
                    match func3 {
                        FUNC3_LB => "lb",
                        FUNC3_LH => "lh",
                        FUNC3_LBU => "lbu",
                        FUNC3_LHU => "lhu",
                        FUNC3_LW => "lw",
                        _ => "",
                    },
                    REG_NAMES[rd as usize],
                    imm as i32,
                    REG_NAMES[rs1 as usize]
                )
            );

            if let Err(why) = rf.write(
                rd,
                match {
                    match func3 {
                        FUNC3_LB => match mem.read_byte(addr) {
                            Ok(d) => Ok((d as i8) as u32), // sign-extension
                            Err(why) => return Err(why),
                        },
                        FUNC3_LH => match mem.read_half_word(addr) {
                            Ok(d) => Ok((d as i16) as u32), // sign-extension
                            Err(why) => return Err(why),
                        },
                        FUNC3_LBU => mem.read_byte(addr),
                        FUNC3_LHU => mem.read_half_word(addr),
                        FUNC3_LW => mem.read_word(addr),
                        _ => return Err(RiscvError::InvalidFunc3Error(ir, func3)),
                    }
                } {
                    Ok(d) => d,
                    Err(why) => return Err(why),
                },
            ) {
                return Err(why);
            }
            *pc += 4;

            info!("\n");
            Ok(())
        }

        OPCODE_STORE => {
            let rs1 = rs1!(ir);
            let rs2 = rs2!(ir);
            let imm = s_imm!(ir);
            let func3 = func3!(ir);

            info!(
                "{:25} |  ",
                format!(
                    "{:6} {}, {}({})",
                    match func3 {
                        FUNC3_SB => "sb",
                        FUNC3_SH => "sh",
                        FUNC3_SW => "sw",
                        _ => "",
                    },
                    REG_NAMES[rs2 as usize],
                    imm as i32,
                    REG_NAMES[rs1 as usize]
                )
            );

            let addr = match rf.read(rs1) {
                Ok(d) => d,
                Err(why) => return Err(why),
            }
            .overflowing_add(imm)
            .0;
            let data = match rf.read(rs2) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            if let Err(why) = match func3 {
                FUNC3_SB => mem.write_byte(addr, data),
                FUNC3_SH => mem.write_half_word(addr, data),
                FUNC3_SW => mem.write_word(addr, data),
                _ => Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
            } {
                return Err(why);
            }
            *pc += 4;

            info!("\n");
            Ok(())
        }

        OPCODE_ARITHMETIC | OPCODE_ARITHMETIC_IMM => {
            let rd = rd!(ir);
            let rs1 = rs1!(ir);
            let lhs = match rf.read(rs1) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            let rhs = match opcode {
                OPCODE_ARITHMETIC => match rf.read(rs2!(ir)) {
                    Ok(d) => d,
                    Err(why) => return Err(why),
                },
                OPCODE_ARITHMETIC_IMM => i_imm!(ir),
                _ => return Err(RiscvError::InvalidOpcodeError(ir, opcode!(ir))),
            };
            let ir_name: &str;
            let bi_operator = match func3!(ir) {
                // This func3 is complicated, it depends on whether we're
                // using immediates or not.
                FUNC3_ADD_SUB => match opcode {
                    OPCODE_ARITHMETIC => match func3!(ir) {
                        FUNC7_SUB => {
                            ir_name = "sub";
                            |l: u32, r: u32| l.overflowing_sub(r).0
                        }
                        FUNC7_ADD => {
                            ir_name = "add";
                            |l: u32, r: u32| l.overflowing_add(r).0
                        }
                        _ => return Err(RiscvError::InvalidFunc7Error(ir, func7!(ir))),
                    },
                    OPCODE_ARITHMETIC_IMM => {
                        ir_name = "add";
                        |l: u32, r: u32| l.overflowing_add(r).0
                    }
                    _ => return Err(RiscvError::InvalidOpcodeError(ir, opcode!(ir))),
                },
                FUNC3_SLL => {
                    ir_name = "sll";
                    |l: u32, r: u32| l << r
                }
                FUNC3_SLT => {
                    ir_name = "slt";
                    // sign-extension
                    |l: u32, r: u32| if (l as i32) < (r as i32) { 1 } else { 0 }
                }
                FUNC3_SLTU => {
                    ir_name = "sltu";
                    |l: u32, r: u32| if l < r { 1 } else { 0 }
                }
                FUNC3_XOR => {
                    ir_name = "xor";
                    |l: u32, r: u32| l ^ r
                }
                FUNC3_SRA_SRL => match func7!(ir) {
                    FUNC7_SRA => {
                        ir_name = "sra";
                        |l: u32, r: u32| ((l as i32) >> r) as u32 // sign-extension
                    }
                    FUNC7_SRL => {
                        ir_name = "srl";
                        |l: u32, r: u32| l >> r
                    }
                    _ => return Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
                },
                FUNC3_OR => {
                    ir_name = "or";
                    |l: u32, r: u32| l | r
                }
                FUNC3_AND => {
                    ir_name = "and";
                    |l: u32, r: u32| l & r
                }
                _ => return Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
            };

            info!(
                "{:25} |  ",
                format!(
                    "{:6} {}, {}, {}",
                    ir_name.to_owned()
                        + match opcode {
                            OPCODE_ARITHMETIC => "",
                            OPCODE_ARITHMETIC_IMM => "i",
                            _ => "?",
                        },
                    REG_NAMES[rd as usize],
                    REG_NAMES[rs1 as usize],
                    match opcode {
                        OPCODE_ARITHMETIC => String::from(REG_NAMES[rs2!(ir) as usize]),
                        OPCODE_ARITHMETIC_IMM => (i_imm!(ir) as i32).to_string(),
                        _ => String::from("?"),
                    }
                )
            );

            if let Err(why) = rf.write(rd!(ir), bi_operator(lhs, rhs)) {
                return Err(why);
            }
            *pc += 4;

            info!("\n");
            Ok(())
        }
        _ => Err(RiscvError::InvalidOpcodeError(ir, opcode!(ir))),
    };
}
