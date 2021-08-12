use lib_rv32::{constants::*, *};
mod instructions;

macro_rules! assert_eq {
    ($a:expr, $b:expr) => {
        std::assert_eq!($a, $b, "\n{:032b}\n{:032b}", $a, $b)
    };
}

#[test]
fn test_decode_opcode() {
    assert_eq!(OPCODE_LUI, decode_opcode!(instructions::LUI_X5_4));
    assert_eq!(OPCODE_AUIPC, decode_opcode!(instructions::AUIPC_X5_4));
    assert_eq!(OPCODE_JAL, decode_opcode!(instructions::JAL_X0_16));
    assert_eq!(OPCODE_JALR, decode_opcode!(instructions::JALR_X5_X5_4));
    assert_eq!(OPCODE_BRANCH, decode_opcode!(instructions::BEQ_X5_X5_12));
    assert_eq!(OPCODE_LOAD, decode_opcode!(instructions::LW_X5_0_X5));
    assert_eq!(OPCODE_STORE, decode_opcode!(instructions::SW_X5_0_X5));
}

#[test]
fn test_decode_func3() {
    assert_eq!(FUNC3_BEQ, decode_func3!(instructions::BEQ_X5_X5_12));
    assert_eq!(FUNC3_BNE, decode_func3!(instructions::BNE_X5_X5_76));
    assert_eq!(FUNC3_BLT, decode_func3!(instructions::BLT_X5_X5_72));
    assert_eq!(FUNC3_BGEU, decode_func3!(instructions::BGEU_X5_X5_68));
    assert_eq!(FUNC3_LB, decode_func3!(instructions::LB_X5_0_X5));
    assert_eq!(FUNC3_LBU, decode_func3!(instructions::LBU_X5_0_X5));
    assert_eq!(FUNC3_LH, decode_func3!(instructions::LH_X5_0_X5));
    assert_eq!(FUNC3_LHU, decode_func3!(instructions::LHU_X5_0_X5));
    assert_eq!(FUNC3_SB, decode_func3!(instructions::SB_X5_0_X5));
    assert_eq!(FUNC3_SH, decode_func3!(instructions::SH_X5_0_X5));
    assert_eq!(FUNC3_SW, decode_func3!(instructions::SW_X5_0_X5));
    assert_eq!(FUNC3_ADD_SUB, decode_func3!(instructions::ADDI_X0_X0_17));
    assert_eq!(FUNC3_ADD_SUB, decode_func3!(instructions::SUB_X5_X5_X5));
    assert_eq!(FUNC3_SLL, decode_func3!(instructions::SLLI_X5_X5_1));
    assert_eq!(FUNC3_SLT, decode_func3!(instructions::SLTI_X5_X5_1));
    assert_eq!(FUNC3_SLTU, decode_func3!(instructions::SLTU_X5_X5_X5));
    assert_eq!(FUNC3_XOR, decode_func3!(instructions::XORI_X5_X5_1));
    assert_eq!(FUNC3_SR, decode_func3!(instructions::SRAI_X5_X5_1));
    assert_eq!(FUNC3_OR, decode_func3!(instructions::ORI_X5_X5_1));
    assert_eq!(FUNC3_AND, decode_func3!(instructions::ANDI_X5_X5_1));
}

#[test]
fn test_decode_i_imm() {
    assert_eq!(17, decode_i_imm!(instructions::ADDI_X0_X0_17));
    assert_eq!(82, decode_i_imm!(instructions::XORI_X5_X6_82));
    assert_eq!(0, decode_i_imm!(instructions::ADDI_X5_X6_0));
    assert_eq!(2047, decode_i_imm!(instructions::ADDI_X5_X6_2047));
    assert_eq!(-12, decode_i_imm!(instructions::ADDI_X5_X6_NEG_12) as i32);
    assert_eq!(-1, decode_i_imm!(instructions::ADDI_X5_X6_NEG_1) as i32);
    assert_eq!(
        -2048,
        decode_i_imm!(instructions::ADDI_X5_X6_NEG_2048) as i32
    );
}

#[test]
fn test_decode_j_imm() {
    assert_eq!(-8, decode_j_imm!(instructions::JAL_X0_NEG_8) as i32);
    assert_eq!(16, decode_j_imm!(instructions::JAL_X0_16));
}

#[test]
fn test_decode_b_imm() {}

#[test]
fn test_decode_s_imm() {
    assert_eq!(0, decode_s_imm!(instructions::SW_X5_0_X5));
    assert_eq!(16, decode_s_imm!(instructions::SW_X5_16_X5));
    assert_eq!(-40, decode_s_imm!(instructions::SW_X5_NEG_40_X5) as i32);
    assert_eq!(-36, decode_s_imm!(instructions::SW_A0_NEG_36_SP) as i32);
    assert_eq!(-20, decode_s_imm!(instructions::SW_A0_NEG_20_S0) as i32);
}

#[test]
fn test_decode_rs1() {
    for i in 0..32 {
        assert_eq!(
            i as u8,
            decode_rs1!(instructions::ADD_SAME_REG_FIELDS_IRS[i])
        );
    }
}

#[test]
fn test_decode_rs2() {
    for i in 0..32 {
        assert_eq!(
            i as u8,
            decode_rs2!(instructions::ADD_SAME_REG_FIELDS_IRS[i])
        );
    }
}

#[test]
fn test_decode_rd() {
    for i in 0..32 {
        assert_eq!(
            i as u8,
            decode_rd!(instructions::ADD_SAME_REG_FIELDS_IRS[i])
        );
    }
}
