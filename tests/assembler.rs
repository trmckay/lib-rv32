mod instructions;
use lib_rv32::{assembler::*, constants::*, *};
use std::collections::HashMap;

macro_rules! assert_eq {
    ($a:expr, $b:expr) => {
        std::assert_eq!($a, $b, "\n{:032b}\n{:032b}", $a, $b)
    };
}

#[test]
fn test_assemble_copious_commas() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi,, t0,,, x6,, 0,,,", &mut empty_hash).unwrap()
    );
}

#[test]
fn test_assemble_no_commas() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0 x6 0", &mut empty_hash).unwrap()
    );
}

#[test]
fn test_assemble_uppercase() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("ADDI T0, X6, 0", &mut empty_hash).unwrap()
    );
}

#[test]
fn test_assemble_random_case() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("aDdI t0, X6, 0", &mut empty_hash).unwrap()
    );
}

#[test]
fn test_assemble_lower_case() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0, x6, 0", &mut empty_hash).unwrap()
    );
}

#[test]
fn test_assemble_i_type() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0, x6, 0", &mut empty_hash).unwrap()
    );
    assert_eq!(
        instructions::ADDI_X0_X0_17,
        assemble_ir("addi zero, x0, 17", &mut empty_hash).unwrap()
    );
    assert_eq!(
        instructions::ADDI_X5_X6_NEG_12,
        assemble_ir("addi t0, t1, -12", &mut empty_hash).unwrap()
    );
    assert_eq!(
        instructions::LW_X5_0_X5,
        assemble_ir("lw x5, 0(x5)", &mut empty_hash).unwrap()
    )
}

#[test]
fn test_assemble_u_type() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::AUIPC_X5_4,
        assemble_ir("auipc x5, 4", &mut empty_hash).unwrap()
    );
    assert_eq!(
        instructions::LUI_X5_4,
        assemble_ir("lui x5, 4", &mut empty_hash).unwrap()
    );
}

#[test]
fn test_assemble_b_type() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();

    let expect = instructions::BEQ_X5_X5_12;
    let actual = assemble_ir("beq x5, x5, 12", &mut empty_hash).unwrap();
    assert_eq!(expect, actual);

    let expect = instructions::BNE_X5_X5_76;
    let actual = assemble_ir("bne x5, x5, 76", &mut empty_hash).unwrap();
    assert_eq!(expect, actual);
}

macro_rules! test_field {
    ($field:expr,$expect:expr) => {
        assert_eq!($expect, $field | $expect)
    };
}

#[test]
fn test_encode_b_imm() {
    test_field!(encode_b_imm!(72), instructions::BLT_X5_X5_72);
    test_field!(encode_b_imm!(76), instructions::BNE_X5_X5_76);
}

#[test]
fn test_encode_i_imm() {
    test_field!(encode_i_imm!(17), instructions::ADDI_X0_X0_17);
    test_field!(
        encode_i_imm!(-(2048 as i32) as u32),
        instructions::ADDI_X5_X6_NEG_2048
    );
}

#[test]
fn test_encode_rs1() {
    test_field!(encode_rs1!(5), instructions::BEQ_X5_X5_12);
    test_field!(encode_rs1!(5), instructions::BNE_X5_X5_76);
}

#[test]
fn test_encode_rs2() {
    test_field!(encode_rs2!(5), instructions::BEQ_X5_X5_12);
    test_field!(encode_rs2!(5), instructions::BNE_X5_X5_76);
}

#[test]
fn test_encode_func3() {
    test_field!(encode_func3!(FUNC3_BEQ), instructions::BEQ_X5_X5_12);
    test_field!(encode_func3!(FUNC3_BNE), instructions::BNE_X5_X5_76);
}
