use std::path::PathBuf;

use bump::BumpVersion;
use clap::{Parser, Subcommand};

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
}

#[derive(Subcommand)]
enum Commands {
    Bump {
        #[arg(long)]
        major: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        // Command::Transpile => ts_transpile::run(),
        // Command::Minify => minify_js::run(),
        Command::Bump { path, r#type } => bump::bump_pack(bump::BumpArgs { path, r#type }),
        // Command::Pack => packager::run(),
        // Command::Validate => json_schema::run(),
    };
}
