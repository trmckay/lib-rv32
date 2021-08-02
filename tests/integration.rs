mod instructions;
use lib_rv32::mcu::Mcu;
use lib_rv32::{exec_one, Memory, RegisterFile};

const MEM_SIZE: u32 = 1024;

#[test]
fn program_mcu() {
    let mut mcu = Mcu::new(MEM_SIZE as usize);
    mcu.mem
        .program_words(instructions::MULTIPLY_PROGRAM)
        .unwrap();

    for (i, w) in instructions::MULTIPLY_PROGRAM.iter().enumerate() {
        assert_eq!(*w, mcu.mem.read_word((i * 4) as u32).unwrap());
    }
    for i in instructions::MULTIPLY_PROGRAM.len() as u32..(MEM_SIZE / 4) {
        assert_eq!(0, mcu.mem.read_word(i * 4).unwrap());
    }
}

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

#[test]
fn simple_loop() {
    let mut mcu = Mcu::new(MEM_SIZE as usize);
    mcu.mem.program_words(instructions::SIMPLE_LOOP).unwrap();

    let mut cycles = 0;
    loop {
        exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();

        if cycles == 0 {
            assert_eq!(4, mcu.rf.read(5).unwrap());
        }

        if mcu.pc == (instructions::SIMPLE_LOOP.len() * 4) as u32 {
            break;
        }

        cycles += 1;
        assert!(cycles < 12);
    }
    assert_eq!(0, mcu.rf.read(5).unwrap());
}

#[test]
fn multiply_program() {
    let mut mcu = Mcu::new(MEM_SIZE as usize);
    mcu.mem
        .program_words(instructions::MULTIPLY_PROGRAM)
        .unwrap();

    // exec_one will error once the PC increments beyond the program.
    let mut cycles = 0;
    loop {
        exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();
        if mcu.pc == (instructions::MULTIPLY_PROGRAM.len() * 4) as u32 {
            break;
        }
        cycles += 1;
        assert!(cycles < 25);
    }

    assert_eq!(20, mcu.rf.read(10).unwrap());
    assert_eq!(0, mcu.rf.read(11).unwrap());
    assert_eq!(4, mcu.rf.read(5).unwrap());
    assert_eq!(16, mcu.rf.read(1).unwrap());
}
