use assert_cmd::Command;

#[test]
fn test_default_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    let cmd = cmd.arg("--command").arg("echo -n hello");
    let assert = cmd.assert();
    assert.success().stdout("hello\n");
    Ok(())
}

#[test]
fn test_interval() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--command")
        .arg("echo -n 1")
        .arg("--interval")
        .arg("1")
        .arg("--count")
        .arg("2");
    let assert = cmd.assert();
    assert.success().stdout("1\n1\n");
    let duration = start.elapsed();
    assert!(duration.as_secs() >= 1);
    Ok(())
}

#[test]
fn test_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--command")
        .arg("echo -n test")
        .arg("--count")
        .arg("3");
    let assert = cmd.assert();
    assert.success().stdout("test\ntest\ntest\n");
    Ok(())
}

#[test]
fn test_show() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--command")
        .arg("echo -n show_test")
        .arg("--show")
        .arg("--count")
        .arg("1");
    let assert = cmd.assert();
    assert.success().stdout(predicates::str::contains(
        "iter: 1, every: 0s, command: echo -n show_test\n\nshow_test\n",
    ));
    Ok(())
}

#[test]
fn test_stop_on_error_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--command")
        .arg("echo -n success")
        .arg("--count")
        .arg("2")
        .arg("--stop-on-error");
    let assert = cmd.assert();
    assert.success().stdout("success\nsuccess\n");
    Ok(())
}

#[test]
fn test_stop_on_error_failure() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--command")
        .arg("ls missing_dir")
        .arg("--count")
        .arg("3")
        .arg("--stop-on-error");
    let assert = cmd.assert();
    assert
        .success()
        .stderr(predicates::str::contains(
            "ls: cannot access \'missing_dir\': No such file or directory\n",
        ))
        .stdout(predicates::str::contains(
            "Stopping further iterations due to error.",
        ));
    Ok(())
}

#[test]
fn test_command_with_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--command").arg("echo -n hello world");
    let assert = cmd.assert();
    assert.success().stdout("hello world\n");
    Ok(())
}
