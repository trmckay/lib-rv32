use std::fs;
use std::path::Path;

use crate::util::parse_int;
use crate::REG_NAMES;
use crate::{traits::*, util};

pub struct Assertions {
    pub register_assertions: Vec<(u8, u32, bool)>,
    pub memory_assertions: Vec<(u32, u32, bool)>,
}

impl Assertions {
    pub fn load(path: &Path) -> Self {
        let test_params: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        let mut register_assertions: Vec<(u8, u32, bool)> = Vec::new();
        for (i, name) in REG_NAMES.iter().enumerate() {
            if let Some(s) = test_params["registers"][*name].as_str() {
                let d = parse_int(s).unwrap();
                register_assertions.push((i as u8, d as u32, true));
            }
        }

        let mut memory_assertions: Vec<(u32, u32, bool)> = Vec::new();
        let kvs = test_params["memory"].as_object();
        if kvs.is_some() {
            for (k, v) in kvs.unwrap() {
                let addr = parse_int(k).unwrap();
                let data = parse_int(v.as_str().unwrap()).unwrap();
                memory_assertions.push((addr, data, true));
            }
        }

        Assertions {
            register_assertions,
            memory_assertions,
        }
    }

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
