use std::fs;
use std::path::Path;

use crate::constants::*;
use crate::parse_int;
use crate::traits::*;

/// Used to contain a set of assertions about the state of an MCU.
pub struct Assertions {
    pub register_assertions: Vec<(u8, u32, bool)>,
    pub memory_assertions: Vec<(u32, u32, bool)>,
}

impl Assertions {
    /// Construct an `Assertion` from a JSON file.
    ///
    /// Example:
    ///
    /// In `assert.json`:
    /// ```json
    /// {
    ///     "registers": {
    ///         "a0": "10",
    ///         "t0": "0"
    ///     },
    ///     "memory": {
    ///         "0x1000": "0x10",
    ///         "0x1004": "0x4"
    ///     }
    /// }
    /// ```
    ///
    /// ```no_run
    /// # use lib_rv32::Assertions;
    /// use std::path::PathBuf;
    ///
    /// let asserts = Assertions::load(&PathBuf::from("assert.json"));
    /// ```
    pub fn load(path: &Path) -> Self {
        let test_params: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        let mut register_assertions: Vec<(u8, u32, bool)> = Vec::new();
        for (i, name) in REG_NAMES.iter().enumerate() {
            if let Some(s) = test_params["registers"][*name].as_str() {
                let d = parse_int!(u32, s).unwrap();
                register_assertions.push((i as u8, d as u32, true));
            }
        }

        let mut memory_assertions: Vec<(u32, u32, bool)> = Vec::new();
        let kvs = test_params["memory"].as_object();
        if kvs.is_some() {
            for (k, v) in kvs.unwrap() {
                let addr = parse_int!(u32, k).unwrap();
                let data = parse_int!(u32, v.as_str().unwrap()).unwrap();
                memory_assertions.push((addr, data, true));
            }
        }

        Assertions {
            register_assertions,
            memory_assertions,
        }
    }

    /// Iterate through all assertions and compare their expected values
    /// with the actual. If the assertion fails, the third member of the
    /// tuple is populated with `false`.
    pub fn assert_all<M, R>(&mut self, mem: &mut M, rf: &mut R)
    where
        M: Memory,
        R: RegisterFile,
    {
        for a in self.register_assertions.iter_mut() {
            a.2 = rf.read(a.0).unwrap() == a.1;
        }
        for a in self.memory_assertions.iter_mut() {
            a.2 = mem.fetch(a.0).unwrap() == a.1;
        }
    }
}
