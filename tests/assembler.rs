mod instructions;
use lib_rv32::assembler::*;
use std::collections::HashMap;

#[test]
fn test_copious_commas() {
    let empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi,, t0,,, x6,, 0,,,", &empty_hash).unwrap()
    );
}

#[test]
fn test_no_commas() {
    let empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0 x6 0", &empty_hash).unwrap()
    );
}

#[test]
fn test_uppercase() {
    let empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("ADDI T0, X6, 0", &empty_hash).unwrap()
    );
}

#[test]
fn test_random_case() {
    let empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("aDdI t0, X6, 0", &empty_hash).unwrap()
    );
}

#[test]
fn test_lower_case() {
    let empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0, x6, 0", &empty_hash).unwrap()
    );
}

#[test]
fn test_i_type() {
    let empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::ADDI_X5_X6_0,
        assemble_ir("addi t0, x6, 0", &empty_hash).unwrap()
    );
    assert_eq!(
        instructions::ADDI_X0_X0_17,
        assemble_ir("addi zero, x0, 17", &empty_hash).unwrap()
    );
    assert_eq!(
        instructions::ADDI_X5_X6_NEG_12,
        assemble_ir("addi t0, t1, -12", &empty_hash).unwrap()
    );
    assert_eq!(
        instructions::LW_X5_0_X5,
        assemble_ir("lw x5, 0(x5)", &empty_hash).unwrap()
    )
}

#[test]
fn test_u_type() {
    let empty_hash: HashMap<String, u32> = HashMap::new();
    assert_eq!(
        instructions::AUIPC_X5_4,
        assemble_ir("auipc x5, 4", &empty_hash).unwrap()
    );
    assert_eq!(
        instructions::LUI_X5_4,
        assemble_ir("lui x5, 4", &empty_hash).unwrap()
    );
}
