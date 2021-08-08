use std::fs;
use std::io::BufReader;

use std::path::PathBuf;

use clap::{App, Arg};
use lazy_static::lazy_static;
use log::info;
use log::{Level, LevelFilter, Metadata, Record};

use lib_rv32::{assembler::*, constants::*, exec_one, mcu::*, Assertions};

const DEFAULT_MEM_SIZE: usize = 1024 * 64;

lazy_static! {
    static ref CFG: Config = Config::new();
}

static LOGGER: Logger = Logger;

enum Mode {
    Emulator,
    Assembler,
}

struct Config {
    file: PathBuf,
    mem_size: usize,
    stop_pc: Option<u32>,
    assertions: Option<PathBuf>,
    output: Option<PathBuf>,
    mode: Mode,
}

impl Config {
    fn new() -> Self {
        let matches = App::new("lib-rv32")
            .version("0.2.0")
            .author("Trevor McKay <tm@trmckay.com>")
            .about("Emulate RISC-V")
            .arg(
                Arg::with_name("file")
                    .help("File on which to act")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("mem")
                    .short("m")
                    .long("mem")
                    .value_name("MEM_SIZE")
                    .help("Set the size of the MCU memory (default 64 KB)")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("stop")
                    .short("s")
                    .long("--stop")
                    .value_name("STOP_PC")
                    .help("Set the program counter at which to stop emulation")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("assertions")
                    .short("a")
                    .long("--assertions")
                    .value_name("ASSERTIONS_FILE")
                    .help("A JSON formatted set of assertions")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("--output")
                    .value_name("OUTPUT_FILE")
                    .help("Out-file for binary in assembler mode, or memory dump in emulator mode")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Enable verbose logging")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("assemble")
                    .short("c")
                    .long("--assemble")
                    .help("Launch in assembler mode")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("emulate")
                    .short("e")
                    .long("--emulate")
                    .default_value("e")
                    .help("Launch in emulator mode")
                    .takes_value(false),
            )
            .get_matches();

        let mem_size = match matches.value_of("config") {
            Some(s) => str::parse(s).unwrap(),
            None => DEFAULT_MEM_SIZE,
        };
        let stop_pc = matches.value_of("stop").map(|s| {
            u32::from_str_radix(s, 16)
                .unwrap_or_else(|_| panic!("{} is not a valid hex literal.", s))
        });
        let verbose = !matches!(matches.occurrences_of("verbose"), 0);
        let path = PathBuf::from(matches.value_of("file").unwrap());
        let assertions = matches.value_of("assertions").map(PathBuf::from);
        let output = matches.value_of("output").map(PathBuf::from);

        let mode: Mode = if matches.occurrences_of("emulate") == 1 {
            Mode::Emulator
        } else if matches.occurrences_of("assemble") == 1 {
            Mode::Assembler
        } else {
            panic!("No mode provided.");
        };
        if matches.occurrences_of("emulate") == matches.occurrences_of("assemble") {
            panic!("Cannot launch in both modes.");
        }

        if verbose {
            log::set_logger(&LOGGER)
                .map(|()| log::set_max_level(LevelFilter::Info))
                .unwrap();
        }

        Config {
            file: path,
            mem_size,
            stop_pc,
            assertions,
            mode,
            output,
        }
    }
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print!("{}", record.args());
        }
    }

    fn flush(&self) {}
}

fn emu() {
    let assertions = CFG.assertions.as_ref().map(|p| Assertions::load(p));

    let mut mcu: Mcu = Mcu::new(CFG.mem_size);
    mcu.mem
        .program_from_file(&CFG.file)
        .expect("Could not program MCU.");

    loop {
        exec_one(&mut mcu.pc, &mut mcu.mem, &mut mcu.rf).unwrap();
        if Some(mcu.pc) == CFG.stop_pc {
            info!("\nReached stop-PC.\n");
            break;
        }
    }

    if let Some(mut assertions) = assertions {
        assertions.assert_all(&mut mcu.mem, &mut mcu.rf);
        println!();

        for assert in assertions.register_assertions {
            if assert.2 {
                println!("{} == {}", REG_NAMES[assert.0 as usize], assert.1)
            } else {
                println!(
                    "({} = {}) != {}",
                    REG_NAMES[assert.0 as usize],
                    mcu.rf.read(assert.0).unwrap(),
                    assert.1
                );
            }
        }
        println!();

        for assert in assertions.memory_assertions {
            if assert.2 {
                println!("*0x{:08x} == {}", assert.0, assert.1)
            } else {
                println!(
                    "(*0x{:08x} = {}) != {}",
                    assert.0,
                    mcu.mem.fetch(assert.0).unwrap(),
                    assert.1
                );
            }
        }
    }
}

fn asm() {
    let file = fs::File::open(&CFG.file).unwrap();
    let mut reader = BufReader::new(file);
    let _vec = assemble_buf(&mut reader).unwrap();
}

fn main() {
    match CFG.mode {
        Mode::Assembler => asm(),
        Mode::Emulator => emu(),
    }
}
