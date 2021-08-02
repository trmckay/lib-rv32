use crate::decode::*;
use crate::{b_imm, func3, func7, i_imm, j_imm, opcode, rd, rs1, rs2, u_imm};
use crate::{Memory, RegisterFile, RiscvError};

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
    let ir = mem.read_word(*pc);
    if let Err(why) = ir {
        return Err(why);
    }
    let ir = ir.unwrap();

    let opcode = opcode!(ir) as u8;

    return match opcode {
        OPCODE_LUI => {
            let res = rf.write(rd!(ir), u_imm!(ir));
            *pc += 4;
            res
        }

        OPCODE_AUIPC => {
            let res = rf.write(rd!(ir), *pc + u_imm!(ir));
            *pc += 4;
            res
        }

        OPCODE_JAL => {
            let res = rf.write(rd!(ir), *pc + 4);
            *pc = pc.overflowing_add(j_imm!(ir)).0;
            res
        }

        OPCODE_JALR => {
            let rs1 = match rf.read(rs1!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            let res = rf.write(rd!(ir), *pc + 4);
            *pc = rs1.overflowing_add(i_imm!(ir)).0;
            res
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
                _ => return Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
            } {
                *pc = pc.overflowing_add(b_imm!(ir)).0;
            } else {
                *pc += 4;
            }
            Ok(())
        }

        OPCODE_LOAD => {
            let addr = match rf.read(rs1!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            } + i_imm!(ir);
            let res = rf.write(
                rd!(ir),
                match {
                    match func3!(ir) {
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
                        _ => return Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
                    }
                } {
                    Ok(d) => d,
                    Err(why) => return Err(why),
                },
            );
            *pc += 4;
            res
        }

        OPCODE_STORE => {
            let addr = match rf.read(rs2!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            } + i_imm!(ir);
            let data = match rf.read(rs1!(ir)) {
                Ok(d) => d,
                Err(why) => return Err(why),
            };
            let res = match func3!(ir) {
                FUNC3_SB => mem.write_byte(addr, data),
                FUNC3_SH => mem.write_half_word(addr, data),
                FUNC3_SW => mem.write_word(addr, data),
                _ => Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
            };
            *pc += 4;
            res
        }

        OPCODE_ARITHMETIC | OPCODE_ARITHMETIC_IMM => {
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
                _ => return Err(RiscvError::InvalidOpcodeError(ir, opcode!(ir))),
            };

            let bi_operator = match func3!(ir) {
                // This func3 is complicated, it depends on whether we're
                // using immediates or not.
                FUNC3_ADD_SUB => match opcode {
                    OPCODE_ARITHMETIC => match func3!(ir) {
                        FUNC7_SUB => |l: u32, r: u32| l - r,
                        FUNC7_ADD => |l: u32, r: u32| l.overflowing_add(r).0,
                        _ => return Err(RiscvError::InvalidFunc7Error(ir, func7!(ir))),
                    },
                    OPCODE_ARITHMETIC_IMM => |l: u32, r: u32| l.overflowing_add(r).0,
                    _ => return Err(RiscvError::InvalidOpcodeError(ir, opcode!(ir))),
                },
                FUNC3_SLL => |l: u32, r: u32| l + r,
                FUNC3_SLT => |l: u32, r: u32| if (l as i32) < (r as i32) { 1 } else { 0 }, // sign-extension
                FUNC3_SLTU => |l: u32, r: u32| if l < r { 1 } else { 0 },
                FUNC3_XOR => |l: u32, r: u32| l ^ r,
                FUNC3_SRA_SRL => match func7!(ir) {
                    FUNC7_SRA => |l: u32, r: u32| ((l as i32) >> r) as u32, // sign-extension
                    FUNC7_SRL => |l: u32, r: u32| l >> r,
                    _ => return Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
                },
                FUNC3_OR => |l: u32, r: u32| l | r,
                FUNC3_AND => |l: u32, r: u32| l & r,
                _ => return Err(RiscvError::InvalidFunc3Error(ir, func3!(ir))),
            };

            let res = rf.write(rd!(ir), bi_operator(lhs, rhs));
            *pc += 4;
            res
        }
        _ => Err(RiscvError::InvalidOpcodeError(ir, opcode!(ir))),
    };
}
