use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    // init a mutable cmd from assert_cmd::Command::cargo_bin
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("SAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
    cmd.arg("damn").assert().success();
    Ok(())
}

#[test]
fn compare_n() -> TestResult {
    let outfile = "tests/expected/hello1.n.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    //be careful there is no \n at the end of hello1.n.txt
    cmd.arg("-n")
        .arg("Hello  there")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn compare() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    //be careful there is no \n at the end of hello1.n.txt
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run(args: &[&str], expected_file: &str)->TestResult{
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
#[test]
fn compare1()->TestResult{
    // use std::env;
    // env::set_var("RUST_BACKTRACE", "1");
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn compare2()->TestResult{
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn compare3()->TestResult{
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn compare4()->TestResult{
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}

#[test]
fn compare4copy()->TestResult{
    run(&["-n","Hello", "there"], "tests/expected/hello2.n.txt")
}
