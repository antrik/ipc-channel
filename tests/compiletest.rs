#![cfg(feature = "unstable")]

extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

#[test]
fn compiletest() {
    let mut config = compiletest::default_config();
    config.mode = "compile-fail".parse().unwrap();
    config.src_base = PathBuf::from("tests/compiletest-fail");
    config.target_rustcflags = Some("-L target/debug/ -L target/debug/deps/".to_owned());
    compiletest::run_tests(&config);
}
