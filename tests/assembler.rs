mod instructions;
use lib_rv32::assembler::*;
use lib_rv32::*;
use std::collections::HashMap;

#[test]
fn addi() {
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
}
