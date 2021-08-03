use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    if let Ok(v) = env::var("CI") {
        if v == "1" {
            return;
        }
    }
    let output = process::Command::new("bin/build-tests.sh")
        .output()
        .expect("Failed to execute test build script.");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}
