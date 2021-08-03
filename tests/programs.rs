use glob::glob;

use std::fs;
use std::path::Path;

use lib_rv32::mcu::Mcu;
use lib_rv32::{exec_one, Memory, RegisterFile, RiscvError};

const MEM_SIZE: u32 = 1024 * 64; // 64 KB

struct TestResult {
    name: String,
    dump: String,
    state: Box<Mcu>,
    err: RiscvError,
}

fn run_test(dir: &Path) -> Result<(), TestResult> {
    let test_bin_path_str = format!("{}/prog.bin", dir.display());
    let test_bin = Path::new(&test_bin_path_str);
    let prog_bytes = fs::read(&test_bin).unwrap();

    let test_dump_path_str = format!("{}/dump.txt", dir.display());
    let test_dump_path = Path::new(&test_dump_path_str);

    let test_json_path_str = format!("{}/test_case.json", dir.display());
    let test_json_path = Path::new(&test_json_path_str);
    let test_params: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(test_json_path).unwrap()).unwrap();

    let max_cycles = test_params["max_cycles"].as_u64().unwrap();

    let mut mcu = Mcu::new(MEM_SIZE as usize);
    mcu.mem.program_le_bytes(&prog_bytes).unwrap();

    let mut cycles = 0;

    loop {
        if mcu.pc >= prog_bytes.len() as u32 {
            break;
        }

        if cycles >= max_cycles as u32 {
            break;
        }

        if let Err(why) = exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf) {
            return Err(TestResult {
                name: dir.display().to_string(),
                dump: fs::read_to_string(test_dump_path).unwrap(),
                state: Box::new(mcu.clone()),
                err: why,
            });
        }

        cycles += 1;
    }

    Ok(())
}

#[test]
fn run_test_programs() {
    let mut pass = true;
    for dir in match glob("./tests/programs/*") {
        Err(_) => return,
        Ok(p) => p,
    }
    .map(|p| p.unwrap())
    {
        if dir.is_dir() {
            if let Err(res) = run_test(&dir) {
                pass = false;
                eprintln!("Failed test: {}", res.name);
                eprintln!("Error: {:?}", res.err);
                for rn in 0..32 {
                    eprintln!("x{} = 0x{:08X}", rn, res.state.rf.read(rn as u8).unwrap());
                }
                eprintln!("{}", res.dump);
            } else {
                eprintln!("{}... ok", dir.display());
            }
        }
    }
    assert!(pass);
}
