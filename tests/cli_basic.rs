use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("README Gif Crafter"));
}

#[test]
fn test_markdown_generation_dry_run() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--dry-run", "fake.mp4"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("ffmpeg"));
    // Default width check (implied in dry run log if implemented, or just checking flow)
}

#[test]
fn test_cli_args_parsing() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "--dry-run",
            "fake.mp4",
            "--width",
            "1280",
            "--fps",
            "30",
        ])
        .output()
        .expect("Failed to execute command");

    // In a real integration test we'd check if the args were passed to ffmpeg string
    // But since our main just prints "ffmpeg ...", we can check that.
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("ffmpeg"));
    assert!(stdout.contains("scale=1280:-1:flags=lanczos"));
    // depending on how far we got in pipeline.rs.
    // In the current skeleton `pipeline.rs`, we pushed `-i input`. The fps handling is there.
    // Let's check for fps flag if we implemented it.
    // src/pipeline.rs: "args.push("-r".to_string()); args.push(fps.to_string());"
    // assert!(stdout.contains("-r 30"));
    // Updated to check for filter string
    assert!(stdout.contains("fps=30"));
}

#[test]
fn test_markdown_only() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--markdown-only", "existing_demo.gif"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Generated: existing_demo.gif"));
    assert!(stdout.contains("![Demo](./existing_demo.gif)"));
}
