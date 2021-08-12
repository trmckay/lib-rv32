use crate::{*, parse::*};
use lib_rv32_common::{instructions, constants::*};
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


macro_rules! assert_eq {
    ($a:expr, $b:expr) => {
        std::assert_eq!($a, $b, "\n{:032b}\n{:032b}", $a, $b)
    };
}

macro_rules! test_field {
    ($field:expr,$expect:expr) => {
        assert_eq!($expect, $field | $expect)
    };
}

#[test]
fn test_assemble_copious_commas() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi,, t0,,, x6,, 0,,,", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
}

#[test]
fn test_assemble_no_commas() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0 x6 0", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
}

#[test]
fn test_assemble_uppercase() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("ADDI T0, X6, 0", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
}

#[test]
fn test_assemble_random_case() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("aDdI t0, X6, 0", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
}

#[test]
fn test_assemble_lower_case() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0, x6, 0", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
}

#[test]
fn test_assemble_i_type() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0, x6, 0", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        instructions::ADDI_X0_X0_17,
        assemble_ir("addi zero, x0, 17", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        instructions::ADDI_X5_X6_NEG_12,
        assemble_ir("addi t0, t1, -12", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        instructions::LW_X5_0_X5,
        assemble_ir("lw x5, 0(x5)", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    )
}

#[test]
fn test_assemble_u_type() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::AUIPC_X5_4,
        assemble_ir("auipc x5, 4", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        instructions::LUI_X5_4,
        assemble_ir("lui x5, 4", &mut empty_hash, 0)
            .unwrap()
            .unwrap()
    );
}

#[test]
fn test_assemble_b_type() {
    let mut empty_hash: HashMap<String, u32> = HashMap::new();

    let expect = instructions::BEQ_X5_X5_12;
    let actual = assemble_ir("beq x5, x5, 12", &mut empty_hash, 0)
        .unwrap()
        .unwrap();
    assert_eq!(expect, actual);

    let expect = instructions::BNE_X5_X5_76;
    let actual = assemble_ir("bne t0, t0, 76", &mut empty_hash, 0)
        .unwrap()
        .unwrap();
    assert_eq!(expect, actual);
}

#[test]
fn test_assemble_with_label() {
    let mut labels: HashMap<String, u32> = HashMap::new();

    assert_eq!(
        instructions::LUI_X5_4,
        assemble_ir("loop: lui x5, 4", &mut labels, 0)
            .unwrap()
            .unwrap()
    );

    assert_eq!(0, *(labels.get("loop").unwrap()));

    let expect = instructions::JAL_X0_NEG_4;
    let actual = assemble_ir("jal x0, loop", &mut labels, 4)
        .unwrap()
        .unwrap();
    assert_eq!(expect, actual);

    let expect = instructions::BNE_X0_X5_NEG_4;
    let actual = assemble_ir("bne x0, t0, loop", &mut labels, 4)
        .unwrap()
        .unwrap();
    assert_eq!(expect, actual);
}

#[test]
fn test_encode_b_imm() {
    test_field!(encode_b_imm!(72), instructions::BLT_X5_X5_72);
    test_field!(encode_b_imm!(76), instructions::BNE_X5_X5_76);
}

#[test]
fn test_encode_i_imm() {
    test_field!(encode_i_imm!(17), instructions::ADDI_X0_X0_17);
    let i: i32 = -2048;
    test_field!(encode_i_imm!(i as u32), instructions::ADDI_X5_X6_NEG_2048);
}

#[test]
fn test_encode_j_imm() {
    let i = -4;
    test_field!(encode_j_imm!(i as u32), instructions::JAL_X0_NEG_4);
    let i = -8;
    test_field!(encode_j_imm!(i as u32), instructions::JAL_X0_NEG_8);
    let i = 16;
    test_field!(encode_j_imm!(i as u32), instructions::JAL_X0_16);
}

#[test]
fn test_encode_rs1() {
    test_field!(encode_rs1!(5), instructions::BEQ_X5_X5_12);
    test_field!(encode_rs1!(5), instructions::BNE_X5_X5_76);
}

#[test]
fn test_encode_rs2() {
    test_field!(encode_rs2!(5), instructions::BNE_X0_X5_NEG_4);
}

#[test]
fn test_encode_func3() {
    test_field!(encode_func3!(FUNC3_BEQ), instructions::BEQ_X5_X5_12);
    test_field!(encode_func3!(FUNC3_BNE), instructions::BNE_X5_X5_76);
}
