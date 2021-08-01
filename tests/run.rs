mod instructions;
use lib_rv32i::mcu::Mcu;
use lib_rv32i::{exec_one, RegisterFile};

#[test]
fn addi_x6_x0_1() {
    let mut mcu = Mcu::new(1024);

    let bytes = instructions::ADDI_X5_X5_1.to_le_bytes();
    mcu.mem.program_from_le(&bytes).unwrap();

    exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();

    for i in 0..32 {
        assert_eq!(
            match i {
                5 => 1,
                _ => 0,
            },
            mcu.rf.read(i).unwrap()
        );
    }

    assert_eq!(4, mcu.pc);
}