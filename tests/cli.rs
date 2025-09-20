use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn cargo_bin() -> Command {
    Command::cargo_bin("vscode-kit").expect("binary exists")
}

#[test]
fn cli_generate_all_creates_files() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    cargo_bin()
        .args(["generate", "--project-root", root.to_str().unwrap()])
        .assert()
        .success();

    let vscode = root.join(".vscode");
    assert!(vscode.join("launch.json").is_file());
    assert!(vscode.join("tasks.json").is_file());
    assert!(vscode.join("settings.json").is_file());
}

#[test]
fn cli_generate_selected_subset() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    cargo_bin()
        .args([
            "generate",
            "--project-root",
            root.to_str().unwrap(),
            "--selected",
            "launch,settings",
        ])
        .assert()
        .success();

    let vscode = root.join(".vscode");
    assert!(vscode.join("launch.json").is_file());
    assert!(!vscode.join("tasks.json").exists());
    assert!(vscode.join("settings.json").is_file());
}

#[test]
fn cli_uses_external_template_dir_override() {
    let tmp_root = tempdir().unwrap();
    let project_root = tmp_root.path();

    let tmp_templates = tempdir().unwrap();
    let ext_dir = tmp_templates.path().join("python");
    fs::create_dir_all(&ext_dir).unwrap();
    fs::write(
        ext_dir.join("tasks.json"),
        "{\n  \"version\": \"2.0\", \n  \"tasks\": [{\"label\": \"custom\"}]\n}\n",
    )
    .unwrap();

    cargo_bin()
        .args([
            "generate",
            "--project-root",
            project_root.to_str().unwrap(),
            "--template-dir",
            tmp_templates.path().to_str().unwrap(),
            "--selected",
            "tasks",
        ])
        .assert()
        .success();

    let written = fs::read_to_string(project_root.join(".vscode/tasks.json")).unwrap();
    assert!(written.contains("\"custom\""));
}

#[test]
fn cli_errors_on_missing_project_root() {
    let tmp = tempdir().unwrap();
    let missing = tmp.path().join("no_such_dir");

    cargo_bin()
        .args(["generate", "--project-root", missing.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}
