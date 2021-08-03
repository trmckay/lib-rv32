mod instructions;
use lib_rv32::mcu::Mcu;
use lib_rv32::{exec_one, Memory, RegisterFile};

const MEM_SIZE: u32 = 1024 * 64; // 64 KB

#[test]
fn addi_x5_x5_1() {
    let mut mcu = Mcu::new(MEM_SIZE as usize);
    let bytes = instructions::ADDI_X5_X5_1.to_le_bytes();
    mcu.mem.program_le_bytes(&bytes).unwrap();
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

    for i in 1..(MEM_SIZE / 4) {
        assert_eq!(0, mcu.mem.read_word(i * 4).unwrap());
    }

    assert_eq!(4, mcu.pc);
}

#[test]
fn addi_x5_x6_neg_1() {
    let mut mcu = Mcu::new(MEM_SIZE as usize);
    let bytes = instructions::ADDI_X5_X6_NEG_1.to_le_bytes();
    mcu.mem.program_le_bytes(&bytes).unwrap();
    exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();

    for i in 0..32 {
        assert_eq!(
            match i {
                5 => -1,
                _ => 0,
            },
            mcu.rf.read(i).unwrap() as i32
        );
    }

    for i in 1..(MEM_SIZE / 4) {
        assert_eq!(0, mcu.mem.read_word(i * 4).unwrap());
    }

    assert_eq!(4, mcu.pc);
}
