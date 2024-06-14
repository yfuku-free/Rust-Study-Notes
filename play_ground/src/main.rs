/* Rust-Study-Notes */

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name= "Rust-Study-Notes")]
#[command(author= "y.fukuoka")]
#[command(version= "0.0.0")]
#[command(about= "An application to test what I've learned in Rust", long_about= None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
