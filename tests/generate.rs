use std::fs;
use std::path::Path;

use tempfile::tempdir;
use vscode_kit::generator::GenerationSummary;
use vscode_kit::generator::generate;
use vscode_kit::templates::Preset;
use vscode_kit::templates::TemplateKind;

fn read_to_string<P: AsRef<Path>>(p: P) -> String {
    fs::read_to_string(p).expect("read file")
}

#[test]
fn generate_all_creates_files() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    let selected = [
        TemplateKind::Launch,
        TemplateKind::Tasks,
        TemplateKind::Settings,
    ];
    let summary: GenerationSummary = generate(root, &selected, Preset::PythonBasic, None);

    assert!(
        summary.errors.is_empty(),
        "unexpected errors: {:?}",
        summary.errors
    );
    assert_eq!(summary.created.len(), 3);
    assert_eq!(summary.skipped.len(), 0);

    let vscode = root.join(".vscode");
    assert!(vscode.is_dir());
    assert!(vscode.join("launch.json").is_file());
    assert!(vscode.join("tasks.json").is_file());
    assert!(vscode.join("settings.json").is_file());
}

#[test]
fn generate_skips_existing_files() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();
    let vscode = root.join(".vscode");
    fs::create_dir_all(&vscode).unwrap();
    let tasks_path = vscode.join("tasks.json");
    fs::write(&tasks_path, "{\n  \"existing\": true\n}\n").unwrap();

    let selected = [
        TemplateKind::Launch,
        TemplateKind::Tasks,
        TemplateKind::Settings,
    ];
    let summary = generate(root, &selected, Preset::PythonBasic, None);

    assert!(summary.errors.is_empty());
    assert_eq!(summary.created.len(), 2, "expected 2 created files");
    assert_eq!(summary.skipped.len(), 1, "expected 1 skipped file");

    // Ensure pre-existing file content is unchanged
    let content = read_to_string(tasks_path);
    assert!(content.contains("\"existing\": true"));
}

#[test]
fn generate_uses_external_template_dir() {
    let tmp_root = tempdir().unwrap();
    let project_root = tmp_root.path();

    let tmp_templates = tempdir().unwrap();
    let ext_dir = tmp_templates.path().join("python");
    fs::create_dir_all(&ext_dir).unwrap();

    // Provide a custom tasks.json that should override the embedded one
    let custom = "{\n  \"version\": \"2.0.0\",\n  \"tasks\": [{ \"label\": \"custom\" }]\n}\n";
    fs::write(ext_dir.join("tasks.json"), custom).unwrap();

    let selected = [TemplateKind::Tasks];
    let summary = generate(
        project_root,
        &selected,
        Preset::PythonBasic,
        Some(tmp_templates.path()),
    );

    assert!(summary.errors.is_empty());
    assert_eq!(summary.created.len(), 1);

    let written = read_to_string(project_root.join(".vscode").join("tasks.json"));
    assert!(
        written.contains("\"custom\""),
        "external template should be used"
    );
}
