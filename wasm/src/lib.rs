use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use lib_rv32_mcu::*;
use lib_rv32_asm::assemble_program;
use lib_rv32_isa::exec_one;

pub const DEFAULT_MEM_SIZE: usize = 1024;

#[wasm_bindgen]
pub struct State {
    mcu: Mcu,
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
        State {
            mcu: Mcu::new(DEFAULT_MEM_SIZE),
        }
    }

    /// Program the MCU with a string program.
    pub fn program(&mut self, program: JsValue) {
        let program: String = program.into_serde().unwrap();
        let words = assemble_program(&program).unwrap();

        for (wa, wd) in words.iter().enumerate() {
            self.mcu.mem.write_word(wa as u32, *wd).unwrap();
        }
    }

    /// Advance the MCU by one instruction.
    pub fn exec_one(&mut self) {
        exec_one(&mut self.mcu.pc, &mut self.mcu.mem, &mut self.mcu.rf);
    }
}
