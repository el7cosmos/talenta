use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    let assert = cmd.arg("--version").assert();
    assert
        .code(0)
        .stdout(predicate::str::contains(PKG_NAME))
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn login() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME).unwrap();

    let assert = cmd
        .arg("login")
        .arg("--email")
        .arg("admin@example.com")
        .arg("--password")
        .arg("password")
        .assert();
    assert
        .code(predicate::ne(0))
        .stderr(predicate::str::contains("error: Wrong Email or Password"));

    Ok(())
}
