use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_on_no_arg() {
    let mut cmd = Command::cargo_bin("rusty_echo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rusty_echo").unwrap();
    cmd.arg("hello")
        .assert()
        .success();
}