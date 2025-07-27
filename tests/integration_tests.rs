use assert_cmd::Command;

#[test]
fn test_basic_calculation() {
    let mut cmd = Command::cargo_bin("longer").unwrap();
    let output = cmd
        .arg("--start")
        .arg("1990-01-01")
        .arg("--event")
        .arg("2000-01-01")
        .output()
        .unwrap();

    assert!(output.status.success());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}

#[test]
fn test_different_date_range() {
    let mut cmd = Command::cargo_bin("longer").unwrap();
    let output = cmd
        .arg("--start")
        .arg("1985-03-15")
        .arg("--event")
        .arg("2010-07-22")
        .output()
        .unwrap();

    assert!(output.status.success());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}

#[test]
fn test_invalid_start_date() {
    let mut cmd = Command::cargo_bin("longer").unwrap();
    let output = cmd
        .arg("--start")
        .arg("invalid-date")
        .arg("--event")
        .arg("2000-01-01")
        .output()
        .unwrap();

    assert!(!output.status.success());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_invalid_event_date() {
    let mut cmd = Command::cargo_bin("longer").unwrap();
    let output = cmd
        .arg("--start")
        .arg("1990-01-01")
        .arg("--event")
        .arg("not-a-date")
        .output()
        .unwrap();

    assert!(!output.status.success());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_event_before_start() {
    let mut cmd = Command::cargo_bin("longer").unwrap();
    let output = cmd
        .arg("--start")
        .arg("2000-01-01")
        .arg("--event")
        .arg("1990-01-01")
        .output()
        .unwrap();

    assert!(!output.status.success());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("longer").unwrap();
    let output = cmd.arg("--help").output().unwrap();

    assert!(output.status.success());
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}
