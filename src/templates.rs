use std::fs;
use std::path::Path;
use std::path::PathBuf;

use clap::ValueEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum TemplateKind {
    Launch,
    Tasks,
    Settings,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum Preset {
    #[value(name = "python")]
    PythonBasic,
}

impl Preset {
    pub fn dir_name(&self) -> &'static str {
        match self {
            Preset::PythonBasic => "python",
        }
    }
}

pub fn filename_for(kind: TemplateKind) -> &'static str {
    match kind {
        TemplateKind::Launch => "launch.json",
        TemplateKind::Tasks => "tasks.json",
        TemplateKind::Settings => "settings.json",
    }
}

const PYTHON_BASIC_LAUNCH: &str = include_str!("../templates/python/launch.json");
const PYTHON_BASIC_TASKS: &str = include_str!("../templates/python/tasks.json");
const PYTHON_BASIC_SETTINGS: &str = include_str!("../templates/python/settings.json");

pub fn embedded_template(preset: Preset, kind: TemplateKind) -> &'static str {
    match (preset, kind) {
        (Preset::PythonBasic, TemplateKind::Launch) => PYTHON_BASIC_LAUNCH,
        (Preset::PythonBasic, TemplateKind::Tasks) => PYTHON_BASIC_TASKS,
        (Preset::PythonBasic, TemplateKind::Settings) => PYTHON_BASIC_SETTINGS,
    }
}

pub fn load_template(
    preset: Preset,
    kind: TemplateKind,
    template_dir: Option<&Path>,
) -> std::io::Result<String> {
    if let Some(dir) = template_dir {
        let path: PathBuf = dir.join(preset.dir_name()).join(filename_for(kind));
        match fs::read_to_string(&path) {
            Ok(s) => return Ok(s),
            Err(_) => {
                // Fall back to embedded
            }
        }
    }
    Ok(embedded_template(preset, kind).to_string())
}
