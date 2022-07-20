use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(),Box<dyn std::error::Error>>;

#[test]
fn dies_on_no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("rusty_echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn program_runs_properly() -> TestResult {
    let mut cmd = Command::cargo_bin("rusty_echo")?;
    cmd.arg("hello")
        .assert()
        .success();
    Ok(())
}

fn check_expected_output(args:&[&str],file_path:&str) -> TestResult {
    let expected = fs::read_to_string(file_path)?;
    Command::cargo_bin("rusty_echo")?
            .args(args)
            .assert()
            .success()
            .stdout(expected);
    Ok(())
} 

#[test]
fn hello1() -> TestResult {
    check_expected_output(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    check_expected_output(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_noline() -> TestResult {
    check_expected_output(&["Hello there","-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_noline() -> TestResult {
    check_expected_output(&["-n","Hello", "there"], "tests/expected/hello2.n.txt")
}