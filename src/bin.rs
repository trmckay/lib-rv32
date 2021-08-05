use std::path::PathBuf;

use clap::{App, Arg};
use lazy_static::lazy_static;
use log::info;
use log::{Level, LevelFilter, Metadata, Record};

use lib_rv32::{exec_one, mcu::*, Assertions, REG_NAMES};

const DEFAULT_MEM_SIZE: usize = 1024 * 64;

lazy_static! {
    static ref CFG: Config = Config::new();
}

static LOGGER: Logger = Logger;

struct Config {
    verbose: bool,
    binary: PathBuf,
    mem_size: usize,
    stop_pc: Option<u32>,
    assertions: Option<PathBuf>,
}

impl Config {
    fn new() -> Self {
        let matches = App::new("lib-rv32")
            .version("1.0")
            .author("Trevor McKay <tm@trmckay.com>")
            .about("Emulate RISC-V")
            .arg(
                Arg::with_name("binary")
                    .help("RISC-V binary to execute")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("mem")
                    .short("m")
                    .long("mem")
                    .value_name("MEM_SIZE")
                    .help("Set the size of the MCU memory (default 64 KB).")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("stop")
                    .short("s")
                    .long("--stop")
                    .value_name("STOP_PC")
                    .help("Set the program counter at which to stop emulation.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("assertions")
                    .short("a")
                    .long("--assertions")
                    .value_name("ASSERTIONS")
                    .help("A JSON formatted set of assertions.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Enable verbose logging"),
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
        let path = PathBuf::from(matches.value_of("binary").unwrap());
        let assertions = matches.value_of("assertions").map(PathBuf::from);

        Config {
            verbose,
            binary: path,
            mem_size,
            stop_pc,
            assertions,
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

fn main() {
    if CFG.verbose {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
            .unwrap();
    }

    let assertions = CFG.assertions.as_ref().map(|p| Assertions::load(p));

    let mut mcu: Mcu = Mcu::new(CFG.mem_size);
    mcu.mem
        .program_from_file(&CFG.binary)
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
