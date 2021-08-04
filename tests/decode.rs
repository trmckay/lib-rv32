use lib_rv32::*;
mod instructions;

#[test]
fn opcode() {
    assert_eq!(decode::OPCODE_LUI, opcode!(instructions::LUI_X5_4));
    assert_eq!(decode::OPCODE_AUIPC, opcode!(instructions::AUIPC_X5_4));
    assert_eq!(decode::OPCODE_JAL, opcode!(instructions::JAL_X0_16));
    assert_eq!(decode::OPCODE_JALR, opcode!(instructions::JALR_X5_X5_4));
    assert_eq!(decode::OPCODE_BRANCH, opcode!(instructions::BEQ_X5_X5_12));
    assert_eq!(decode::OPCODE_LOAD, opcode!(instructions::LW_X5_0_X5));
    assert_eq!(decode::OPCODE_STORE, opcode!(instructions::SW_X5_0_X5));
}

#[test]
fn func3() {
    assert_eq!(decode::FUNC3_BEQ, func3!(instructions::BEQ_X5_X5_12));
    assert_eq!(decode::FUNC3_BNE, func3!(instructions::BNE_X5_X5_76));
    assert_eq!(decode::FUNC3_BLT, func3!(instructions::BLT_X5_X5_72));
    assert_eq!(decode::FUNC3_BGEU, func3!(instructions::BGEU_X5_X5_68));
    assert_eq!(decode::FUNC3_LB, func3!(instructions::LB_X5_0_X5));
    assert_eq!(decode::FUNC3_LBU, func3!(instructions::LBU_X5_0_X5));
    assert_eq!(decode::FUNC3_LH, func3!(instructions::LH_X5_0_X5));
    assert_eq!(decode::FUNC3_LHU, func3!(instructions::LHU_X5_0_X5));
    assert_eq!(decode::FUNC3_SB, func3!(instructions::SB_X5_0_X5));
    assert_eq!(decode::FUNC3_SH, func3!(instructions::SH_X5_0_X5));
    assert_eq!(decode::FUNC3_SW, func3!(instructions::SW_X5_0_X5));
    assert_eq!(decode::FUNC3_ADD_SUB, func3!(instructions::ADDI_X0_X0_17));
    assert_eq!(decode::FUNC3_ADD_SUB, func3!(instructions::SUB_X5_X5_X5));
    assert_eq!(decode::FUNC3_SLL, func3!(instructions::SLLI_X5_X5_1));
    assert_eq!(decode::FUNC3_SLT, func3!(instructions::SLTI_X5_X5_1));
    assert_eq!(decode::FUNC3_SLTU, func3!(instructions::SLTU_X5_X5_X5));
    assert_eq!(decode::FUNC3_XOR, func3!(instructions::XORI_X5_X5_1));
    assert_eq!(decode::FUNC3_SRA_SRL, func3!(instructions::SRAI_X5_X5_1));
    assert_eq!(decode::FUNC3_OR, func3!(instructions::ORI_X5_X5_1));
    assert_eq!(decode::FUNC3_AND, func3!(instructions::ANDI_X5_X5_1));
}

#[test]
fn i_imm() {
    assert_eq!(17, i_imm!(instructions::ADDI_X0_X0_17));
    assert_eq!(82, i_imm!(instructions::XORI_X5_X6_82));
    assert_eq!(0, i_imm!(instructions::ADDI_X5_X6_0));
    assert_eq!(2047, i_imm!(instructions::ADDI_X5_X6_2047));
    assert_eq!(-12, i_imm!(instructions::ADDI_X5_X6_NEG_12) as i32);
    assert_eq!(-1, i_imm!(instructions::ADDI_X5_X6_NEG_1) as i32);
    assert_eq!(-2048, i_imm!(instructions::ADDI_X5_X6_NEG_2048) as i32);
}

#[test]
fn j_imm() {
    assert_eq!(-8, j_imm!(instructions::JAL_X0_NEG_8) as i32);
    assert_eq!(16, j_imm!(instructions::JAL_X0_16));
}

#[test]
fn b_imm() {}

#[test]
fn s_imm() {
    assert_eq!(0, s_imm!(instructions::SW_X5_0_X5));
    assert_eq!(16, s_imm!(instructions::SW_X5_16_X5));
    assert_eq!(-40, s_imm!(instructions::SW_X5_NEG_40_X5) as i32);
    assert_eq!(-36, s_imm!(instructions::SW_A0_NEG_36_SP) as i32);
    assert_eq!(-20, s_imm!(instructions::SW_A0_NEG_20_S0) as i32);
}

#[test]
fn rs1() {
    for i in 0..32 {
        assert_eq!(i as u8, rs1!(instructions::ADD_SAME_REG_FIELDS_IRS[i]));
    }
}

#[test]
fn rs2() {
    for i in 0..32 {
        assert_eq!(i as u8, rs2!(instructions::ADD_SAME_REG_FIELDS_IRS[i]));
    }
}

#[test]
fn rd() {
    for i in 0..32 {
        assert_eq!(i as u8, rd!(instructions::ADD_SAME_REG_FIELDS_IRS[i]));
    }
}
