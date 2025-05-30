use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn cmd_runs() {
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.assert().failure(); // No arguments should fail
}

#[test]
fn help_long_flag() {
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn help_short_flag() {
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn output_long_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let output_file = temp.child("output.mp4");

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--output")
        .arg(output_file.path())
        .assert()
        .failure(); // Expecting failure without fit and video files
}

#[test]
fn output_short_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let output_file = temp.child("output.mp4");

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("-o").arg(output_file.path()).assert().failure(); // Expecting failure without fit and video files
}

#[test]
fn fit_long_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    fit_file.touch().unwrap();

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--fit").arg(fit_file.path()).assert().failure(); // Expecting failure without video files or metrics flag
}

#[test]
fn fit_short_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    fit_file.touch().unwrap();

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("-f").arg(fit_file.path()).assert().failure(); // Expecting failure without video files or metrics flag
}

#[test]
fn video_files_no_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let video1 = temp.child("video1.mp4");
    let video2 = temp.child("video2.mp4");
    video1.touch().unwrap();
    video2.touch().unwrap();

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg(video1.path()).arg(video2.path()).assert().failure(); // Expecting failure without fit file
}

#[test]
fn complete_valid_command() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    let video1 = temp.child("video1.mp4");
    let video2 = temp.child("video2.mp4");
    let output_file = temp.child("output.mp4");

    fit_file.touch().unwrap();
    video1.touch().unwrap();
    video2.touch().unwrap();

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--fit")
        .arg(fit_file.path())
        .arg("--output")
        .arg(output_file.path())
        .arg(video1.path())
        .arg(video2.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(output_file.path().to_string_lossy().as_ref())); // All required arguments provided
}

#[test]
fn complete_valid_command_short_flags() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    let video1 = temp.child("video1.mp4");
    let output_file = temp.child("output.mp4");

    fit_file.touch().unwrap();
    video1.touch().unwrap();

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("-f")
        .arg(fit_file.path())
        .arg("-o")
        .arg(output_file.path())
        .arg(video1.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(output_file.path().to_string_lossy().as_ref())); // All required arguments with short flags
}

#[test]
fn without_output_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    let video1 = temp.child("video1.mp4");
    
    fit_file.touch().unwrap();
    video1.touch().unwrap();
    
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--fit")
        .arg(fit_file.path())
        .arg(video1.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("movie.mp4")); // Should use default output file
}

#[test]
fn without_output_flag_short() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    let video1 = temp.child("video1.mp4");
    
    fit_file.touch().unwrap();
    video1.touch().unwrap();
    
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("-f")
        .arg(fit_file.path())
        .arg(video1.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("movie.mp4")); // Should use default output file
}

#[test]
fn metrics_long_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    fit_file.touch().unwrap();
    
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--fit")
        .arg(fit_file.path())
        .arg("--metrics")
        .assert()
        .success()
        .stdout(predicate::str::contains("Metrics found in FIT file"));
}

#[test]
fn metrics_short_flag() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    fit_file.touch().unwrap();
    
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("-f")
        .arg(fit_file.path())
        .arg("-m")
        .assert()
        .success()
        .stdout(predicate::str::contains("Metrics found in FIT file"));
}

#[test]
fn metrics_with_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    let output_file = temp.child("metrics.txt");
    fit_file.touch().unwrap();
    
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--fit")
        .arg(fit_file.path())
        .arg("--metrics")
        .arg("--output")
        .arg(output_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Metrics found in FIT file"));
}

#[test]
fn metrics_with_videos() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    let video1 = temp.child("video1.mp4");
    fit_file.touch().unwrap();
    video1.touch().unwrap();
    
    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--fit")
        .arg(fit_file.path())
        .arg("--metrics")
        .arg(video1.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Metrics found in FIT file"));
}

#[test]
fn missing_fit_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    let video1 = temp.child("video1.mp4");
    let output_file = temp.child("output.mp4");
    video1.touch().unwrap();

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--output")
        .arg(output_file.path())
        .arg(video1.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("fit"));
}

#[test]
fn missing_video_files_without_metrics() {
    let temp = assert_fs::TempDir::new().unwrap();
    let fit_file = temp.child("activity.fit");
    let output_file = temp.child("output.mp4");
    fit_file.touch().unwrap();

    let mut cmd = Command::cargo_bin("fit-video").unwrap();
    cmd.arg("--fit")
        .arg(fit_file.path())
        .arg("--output")
        .arg(output_file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Video"));
}