pub use log::{Level, LevelFilter, Metadata, Record};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            log(&format!("{}", record.args()));
            // Not thread-safe.
            unsafe {
                crate::CONSOLE_TEXT += &format!("{}\n", record.args());
            }
        }
    }

    fn flush(&self) {}
}
