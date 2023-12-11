use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_current_version() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path();

    // Setup the directory as needed...

    let mut cmd = Command::cargo_bin("current_bin").unwrap();
    let assert = cmd.arg("--manifest-version=2").assert();
    cmd.current_dir(temp_dir_path);

    assert
        .success()
        .stdout(predicate::str::contains("current_bin"));

    // Additional assertions on the directory contents...
}

#[test]
fn test_prior_version() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path();

    // Setup the directory as needed...

    let mut cmd = Command::cargo_bin("current_bin").unwrap();
    let assert = cmd.arg("--manifest-version=1").assert();
    cmd.current_dir(temp_dir_path);

    assert
        .success()
        .stdout(predicate::str::contains("prior_bin"));

    // Additional assertions on the directory contents...
}
