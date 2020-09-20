use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn version() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    let assert = cmd.arg("--version").assert();
    assert
        .code(0)
        .stdout(predicate::str::contains(PKG_NAME))
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    cmd.arg("--help").assert().code(0);

    Ok(())
}

#[test]
fn login() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    cmd.arg("login").arg("--help").assert().code(0);

    Ok(())
}

#[test]
fn attendance() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    cmd.arg("attendance").arg("--help").assert().code(0);

    Ok(())
}

#[test]
fn attendance_checkin() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    cmd.arg("attendance")
        .arg("checkin")
        .arg("--help")
        .assert()
        .code(0);

    Ok(())
}

#[test]
fn attendance_checkout() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    cmd.arg("attendance")
        .arg("checkout")
        .arg("--help")
        .assert()
        .code(0);

    Ok(())
}
