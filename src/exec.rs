use crate::decode::*;
use crate::{b_imm, func3, func7, i_imm, j_imm, opcode, rd, rs1, rs2, u_imm};
use crate::{Memory, RegisterFile, RiscvError};

/// Decode an instruction provided as a byte-slice.
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
    let ir = mem.read_word(*pc).unwrap();

    let opcode = opcode!(ir) as u8;

    return match opcode {
        OPCODE_LUI => {
            *pc += 4;
            rf.write(rd!(ir), u_imm!(ir))
        }

        OPCODE_AUIPC => {
            *pc += 4;
            rf.write(rd!(ir), *pc + u_imm!(ir))
        }

        OPCODE_JAL => {
            *pc += j_imm!(ir);
            rf.write(rd!(ir), *pc + 4)
        }

        OPCODE_JALR => {
            *pc = j_imm!(ir);
            rf.write(rd!(ir), *pc + 4)
        }

        OPCODE_BRANCH => {
            let rs1 = match rf.read(rs1!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            let rs2 = match rf.read(rs2!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            if match func3!(ir) {
                FUNC3_BEQ => rs1 == rs2,
                FUNC3_BNE => rs1 != rs2,
                FUNC3_BLT => rs1 < rs2,
                FUNC3_BGE => rs1 >= rs2,
                FUNC3_BLTU => rs1 < rs2,
                FUNC3_BGEU => rs1 > rs2,
                _ => return Err(RiscvError::InvalidFunctionError),
            } {
                *pc += b_imm!(ir);
            } else {
                *pc += 4;
            }
            Ok(())
        }

        OPCODE_LOAD => {
            *pc += 4;
            let addr = match rf.read(rs1!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            } + i_imm!(ir);
            rf.write(
                rd!(ir),
                match {
                    match func3!(ir) {
                        FUNC3_LB | FUNC3_LBU => mem.read_byte(addr),
                        FUNC3_LH | FUNC3_LHU => mem.read_half_word(addr),
                        FUNC3_LW => mem.read_word(addr),
                        _ => return Err(RiscvError::InvalidFunctionError),
                    }
                } {
                    Ok(d) => d as u32,
                    Err(why) => return Err(why),
                },
            )
        }

        OPCODE_STORE => {
            *pc += 4;
            let addr = match rf.read(rs2!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            } + i_imm!(ir);
            let data = match rf.read(rs1!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            match func3!(ir) {
                FUNC3_SB => mem.write_byte(addr, data),
                FUNC3_SH => mem.write_half_word(addr, data),
                FUNC3_SW => mem.write_word(addr, data),
                _ => return Err(RiscvError::InvalidFunctionError),
            }
        }

        OPCODE_ARITHMETIC | OPCODE_ARITHMETIC_IMM => {
            *pc += 4;
            let lhs = match rf.read(rs1!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };

            let rhs = match opcode {
                OPCODE_ARITHMETIC => match rf.read(rs2!(ir)) {
                    Ok(d) => d,
                    Err(why) => return Err(why),
                },
                OPCODE_ARITHMETIC_IMM => i_imm!(ir),
                _ => return Err(RiscvError::InvalidOpcodeError),
            };

            let bi_operator = match func3!(ir) {
                // This func3 is complicated, it depends on whether we're
                // using immediates or not.
                FUNC3_ADD_SUB => match opcode {
                    OPCODE_ARITHMETIC => match func3!(ir) {
                        FUNC7_SUB => |l: u32, r: u32| l - r,
                        FUNC7_ADD => |l: u32, r: u32| l + r,
                        _ => return Err(RiscvError::InvalidFunctionError),
                    },
                    OPCODE_ARITHMETIC_IMM => |l: u32, r: u32| l + r,
                    _ => return Err(RiscvError::InvalidOpcodeError),
                },
                FUNC3_SLL => |l: u32, r: u32| l + r,
                FUNC3_SLT => |l: u32, r: u32| if l < r { 1 } else { 0 },
                FUNC3_SLTU => |l: u32, r: u32| if l < r { 1 } else { 0 },
                FUNC3_XOR => |l: u32, r: u32| l ^ r,
                FUNC3_SRA_SRL => match func7!(ir) {
                    FUNC7_SRA => |l: u32, r: u32| l >> r,
                    FUNC7_SRL => |l: u32, r: u32| l >> r,
                    _ => return Err(RiscvError::InvalidFunctionError),
                },
                FUNC3_OR => |l: u32, r: u32| l | r,
                FUNC3_AND => |l: u32, r: u32| l & r,
                _ => return Err(RiscvError::InvalidFunctionError),
            };

            rf.write(rd!(ir), bi_operator(lhs, rhs))
        }
        _ => return Err(RiscvError::InvalidOpcodeError),
    };
}
