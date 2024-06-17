/* Rust-Study-Notes */

use clap::Parser;
mod infrastructure;

#[derive(Parser, Debug)]
#[command(name= "Rust-Study-Notes")]
#[command(author= "y.fukuoka")]
#[command(version= "0.0.0")]
#[command(about= "An application to test what I've learned in Rust", long_about= None)]
struct Args {
    /// Task
    #[arg(short, long, value_enum, default_value="all")]
    task: Tasks,
}

fn main() {
    let args = Args::parse();
    match args.task {
        Tasks::All => {
            infrastructure::questions::run()
        },
        Tasks::Task0 => {
            infrastructure::questions::task_number_0::run();
        },
        Tasks::Task1 => {
            // 
        },
        Tasks::Task2 => {
            // 
        },
    }
}

#[derive(clap::ValueEnum, Debug, Clone, Default)]
enum Tasks {
    #[default]
    All,
    Task0,
    Task1,
    Task2
}
