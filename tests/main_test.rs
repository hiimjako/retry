use assert_cmd::Command;

#[test]
fn test_default_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    let cmd = cmd.arg("-n").arg("1").arg("echo -n hello");
    let assert = cmd.assert();
    assert.success().stdout("hello\n");
    Ok(())
}

#[test]
fn test_interval() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--interval")
        .arg("1")
        .arg("--count")
        .arg("2")
        .arg("echo -n 1");
    let assert = cmd.assert();
    assert.success().stdout("1\n1\n");
    let duration = start.elapsed();
    assert!(duration.as_secs() >= 1);
    Ok(())
}

#[test]
fn test_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--count").arg("3").arg("echo -n test");
    let assert = cmd.assert();
    assert.success().stdout("test\ntest\ntest\n");
    Ok(())
}

#[test]
fn test_show() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--show")
        .arg("--count")
        .arg("1")
        .arg("echo -n show_test");
    let assert = cmd.assert();
    let pattern = predicates::str::is_match(
        r"iter: 1, every: 1s, last duration: \d+ms, command: echo -n show_test\n\nshow_test\n",
    )?;
    assert.success().stdout(pattern);
    Ok(())
}

#[test]
fn test_stop_on_error_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--count")
        .arg("2")
        .arg("--stop-on-error")
        .arg("echo -n success");
    let assert = cmd.assert();
    assert.success().stdout("success\nsuccess\n");
    Ok(())
}

#[test]
fn test_stop_on_error_failure() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("retry")?;
    cmd.arg("--count")
        .arg("3")
        .arg("--stop-on-error")
        .arg("ls missing_dir");
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
