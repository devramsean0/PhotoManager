use clap::{Parser, Subcommand};
use inquire::Text;

pub mod utils;

mod commands;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, help = "Path to the output directory, not including the collection name")]
    output_path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long)]
        input_path: Option<String>,
        #[arg(short, long)]
        collection_name: Option<String>,
    },
    Sync {
        #[arg(short, long)]
        collection_name: Option<String>,
    }
}

fn main() {
    let cli = Cli::parse();
    // If no output path is given, bring up a prompt
    let output_path = cli.output_path.unwrap_or_else(|| {
        let input = Text::new("Where is your save location?").with_autocomplete(utils::file_path_autocomplete::FilePathCompleter::default()).prompt();
        match input {
            Ok(path) => path,
            Err(e) => {
                println!("Error: {}", e);
                return String::new();
            }
        } 
    });
    match cli.command {
        Commands::Create { input_path, collection_name } => commands::create::run(input_path, collection_name, output_path),
        Commands::Sync { collection_name } => commands::sync::run(collection_name, output_path),
    }
}