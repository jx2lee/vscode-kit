use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use std::io::IsTerminal;

use crate::generator::GenerationSummary;
use crate::generator::generate;
use crate::generator::generate_with_prompt;
use crate::templates::Preset;
use crate::templates::TemplateKind;

#[derive(Parser, Debug)]
#[command(name = "vscode-kit", version, about = "Manage VSCode Setting files", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List existing templates
    List {
        #[arg(long = "project-root", value_name = "PATH", default_value = ".")]
        project_root: PathBuf,
    },
    /// Generate VS Code template files under <project-root>/.vscode
    Generate {
        /// Project root directory where .vscode will be created
        #[arg(long = "project-root", value_name = "PATH")]
        project_root: PathBuf,

        /// Selected files to create (default: all). Accepts comma-separated or repeated.
        #[arg(long = "selected", value_delimiter = ',', value_enum)]
        selected: Vec<TemplateKind>,

        /// Template preset to use (default: python)
        #[arg(long = "preset", value_enum, default_value = "python")]
        preset: Preset,

        /// Optional external template directory to override embedded templates
        #[arg(long = "template-dir", value_name = "DIR")]
        template_dir: Option<PathBuf>,

        /// Run in interactive mode (default: false)
        #[arg(long = "interactive", default_value = "false")]
        interactive: bool,
    },
    /// Validate templates in the given directory
    Validate {
        #[arg(long = "template-root", value_name = "TEMPLATE_PATH")]
        template_root: PathBuf,
    },
}

pub fn run() -> i32 {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate {
            project_root,
            mut selected,
            preset,
            template_dir,
            interactive,
        } => {
            if !project_root.exists() {
                eprintln!(
                    "Error: project root does not exist: {}",
                    project_root.display()
                );
                return 1;
            }
            if !project_root.is_dir() {
                eprintln!(
                    "Error: project root is not a directory: {}",
                    project_root.display()
                );
                return 1;
            }

            let template_dir_opt = match template_dir {
                Some(dir) => {
                    if !dir.exists() {
                        eprintln!("Error: template dir does not exist: {}", dir.display());
                        return 1;
                    }
                    if !dir.is_dir() {
                        eprintln!("Error: template dir is not a directory: {}", dir.display());
                        return 1;
                    }
                    Some(dir)
                }
                None => None,
            };

            if selected.is_empty() {
                selected = vec![
                    TemplateKind::Launch,
                    TemplateKind::Tasks,
                    TemplateKind::Settings,
                ];
            }

            let auto_prompt = interactive || std::io::stdin().is_terminal();
            let summary: GenerationSummary = if auto_prompt {
                generate_with_prompt(
                    &project_root,
                    &selected,
                    preset,
                    template_dir_opt.as_deref(),
                )
            } else {
                generate(
                    &project_root,
                    &selected,
                    preset,
                    template_dir_opt.as_deref(),
                )
            };

            if !summary.created.is_empty() {
                println!("Created:");
                for p in summary.created {
                    println!("- {}", p.display());
                }
            }
            if !summary.skipped.is_empty() {
                println!("Skipped (already exists):");
                for p in summary.skipped {
                    println!("- {}", p.display());
                }
            }
            if !summary.errors.is_empty() {
                eprintln!("Errors:");
                for (p, e) in summary.errors {
                    eprintln!("- {}: {}", p.display(), e);
                }
                return 1;
            }
            0
        }
        Commands::Validate { template_root } => {
            println!("{:?}", template_root.to_str());
            0
        }
        Commands::List { project_root } => 0,
    }
}
