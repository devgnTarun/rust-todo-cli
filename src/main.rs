#[allow(unused)]
mod models;
mod utils;

// Use clap for command line interface
use clap::{Parser, Subcommand};

// Cli-Parser Implementation
#[derive(Parser, Debug)]
#[command(name = "weilder")]
#[command(about = "A CLI-based todo app", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        desc: String,
    },
    List,
    Edit {
        #[arg(short, long)]
        id: Option<u32>,
        #[arg(short, long, default_value_t = String::new())]
        // Not using option enum for title-desc, rather providing an default_value_t = so we might know "no value provided here"
        title: String,
        #[arg(short, long, default_value_t = String::new())]
        desc: String,
        #[arg(short, long)]
        completed: Option<bool>,
    },
    Delete {
        #[arg(short, long)]
        id: Option<u32>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, desc } => {
            utils::add_task(title, desc);
        }
        Commands::List => {
            utils::list_tasks();
        }
        // For better error display! Added option enum
        Commands::Edit {
            id,
            title,
            desc,
            completed,
        } => match id {
            Some(task_id) => {
                utils::edit_tasks(task_id, title, desc, completed);
            }
            None => {
                println!("Please enter the task id!")
            }
        },
        Commands::Delete { id } => match id {
            Some(task_id) => {
                utils::delete_task(task_id);
            }
            None => {
                println!("Please enter the task id!")
            }
        },
    }
}
