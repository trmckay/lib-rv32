#[cfg(debug_assertions)]
use std::{
    io::{self, Write},
    process,
};

#[cfg(debug_assertions)]
fn build_tests() {
    let output = process::Command::new("./build-tests.sh")
        .output()
        .expect("Failed to execute test build script.");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}

fn main() {
    #[cfg(debug_assertions)]
    build_tests();
}
