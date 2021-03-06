use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn missing_pattern_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("").arg("test.txt");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Missing required parameter 'pattern'",
    ));

    Ok(())
}

#[test]
fn missing_file_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foo").arg("");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Missing required parameter 'path'",
    ));

    Ok(())
}
