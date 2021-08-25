use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use lib_rv32_asm::assemble_program;
use lib_rv32_common::constants::*;
use lib_rv32_isa::exec_one;
use lib_rv32_mcu::*;

mod logger;
use logger::*;

pub const DEFAULT_MEM_SIZE: usize = 1024;

static LOGGER: Logger = Logger;

#[wasm_bindgen]
pub struct State {
    mcu: Mcu,
    text_size: usize,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Program {
    asm: String,
}

#[wasm_bindgen]
impl State {
    /// Initiate the MCU state.
    pub fn new() -> Self {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
            .unwrap();

        let mcu = Mcu::new(DEFAULT_MEM_SIZE);
        info!(
            "Initialized RISC-V rv32i MCU with {}k memory.",
            mcu.mem.size / 1024
        );

        State { mcu, text_size: 0 }
    }

    /// Program the MCU with a string program.
    pub fn assemble(&mut self, program: String) {
        let program = program.replace("\\n", "\n");

        let words = assemble_program(&program);

        if words.is_err() {
            info!("Assembler error: {:?}", words);
            return;
        } else {
            info!("Successfully assembled program.");
            info!(" ");
            let words = words.unwrap();
            self.mcu.mem.program_words(&words).unwrap();
            self.text_size = words.len() * 4;
        }
    }

    pub fn run(&mut self) {
        while self.mcu.pc < self.text_size as u32 {
            if let Err(why) = exec_one(&mut self.mcu.pc, &mut self.mcu.mem, &mut self.mcu.rf) {
                info!("MCU runtime error: {:?}", why);
                return;
            }
        }

        info!("Program complete.");
    }

    pub fn get_text(&self) -> String {
        let mut text = String::new();
        for wa in 0..self.text_size / 4 {
            text += &format!("{:08x}\n", self.mcu.mem.fetch((wa as u32) * 4).unwrap());
        }
        text
    }

    pub fn get_state(&self) -> String {
        let mut state = String::new();

        for i in 0..32 {
            let val = self.mcu.rf.read(i as u8).unwrap();
            state += &format!("{:4} = 0x{:08x} ({})\n", REG_NAMES[i], val, val as i32);
        }

        state
    }
}
