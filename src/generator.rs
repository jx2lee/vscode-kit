use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::templates::Preset;
use crate::templates::TemplateKind;
use crate::templates::load_template;

pub struct GenerationSummary {
    pub created: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
    pub errors: Vec<(PathBuf, String)>,
}

enum OverwritePolicy {
    Skip,
    Prompt,
}

pub fn generate(
    project_root: &Path,
    selected: &[TemplateKind],
    preset: Preset,
    template_dir: Option<&Path>,
) -> GenerationSummary {
    generate_impl(
        project_root,
        selected,
        preset,
        template_dir,
        OverwritePolicy::Skip,
    )
}

pub fn generate_with_prompt(
    project_root: &Path,
    selected: &[TemplateKind],
    preset: Preset,
    template_dir: Option<&Path>,
) -> GenerationSummary {
    generate_impl(
        project_root,
        selected,
        preset,
        template_dir,
        OverwritePolicy::Prompt,
    )
}

fn generate_impl(
    project_root: &Path,
    selected: &[TemplateKind],
    preset: Preset,
    template_dir: Option<&Path>,
    overwrite: OverwritePolicy,
) -> GenerationSummary {
    let vscode_dir = project_root.join(".vscode");
    let mut summary = GenerationSummary {
        created: Vec::new(),
        skipped: Vec::new(),
        errors: Vec::new(),
    };

    if let Err(e) = fs::create_dir_all(&vscode_dir) {
        summary
            .errors
            .push((vscode_dir.clone(), format!("failed to create .vscode: {e}")));
        return summary;
    }

    for kind in selected {
        let filename = crate::templates::filename_for(*kind);
        let path = vscode_dir.join(filename);
        let exists = path.exists();
        let should_write = if exists {
            match overwrite {
                OverwritePolicy::Skip => false,
                OverwritePolicy::Prompt => prompt_overwrite(&path),
            }
        } else {
            true
        };

        if !should_write {
            summary.skipped.push(path);
            continue;
        }

        match load_template(preset, *kind, template_dir) {
            Ok(content) => match write_file(&path, &content) {
                Ok(_) => summary.created.push(path),
                Err(e) => summary.errors.push((path, e.to_string())),
            },
            Err(e) => summary.errors.push((path, e.to_string())),
        }
    }

    summary
}

fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn prompt_overwrite(path: &Path) -> bool {
    use std::io::Write;
    use std::io::{self};
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    loop {
        let _ = write!(
            stdout,
            "File exists: {}. Overwrite? [y/N]: ",
            path.display()
        );
        let _ = stdout.flush();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            return false;
        }
        let ans = input.trim().to_lowercase();
        match ans.as_str() {
            "y" | "yes" => return true,
            "n" | "no" | "" => return false,
            _ => {
                // Ask again on unrecognized input
            }
        }
    }
}
