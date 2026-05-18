use bump::BumpVersion;
use chrono::Local;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Bump {
        #[arg(long)]
        path: PathBuf,
        r#type: BumpVersion,
    },
    Init {
        #[arg(long)]
        path: PathBuf,
        template: String,
        name: String,
        description: String,
    },
}

pub fn track_pipeline<F, R>(task_name: &str, pipeline_action: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();

    // Execute the passed compilation closure
    let result = pipeline_action();

    let duration = start.elapsed();
    let current_time = Local::now().format("%H:%M:%S").to_string();

    println!(
        "[{}] \x1b[32mSUCCESS:\x1b[0m {} completed in {:?}",
        current_time, task_name, duration
    );

    result
}

fn main() {
    track_pipeline("run", read_cli);
}

fn read_cli() {
    let cli = Cli::parse();

    match cli.command {
        Command::Bump { path, r#type } => bump::bump_pack(bump::BumpArgs { path, r#type }),
        Command::Init {
            path,
            template,
            name,
            description,
        } => init::handle_init_command(&template, path, &name, &description),
    };
}
