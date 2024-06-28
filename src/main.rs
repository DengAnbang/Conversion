
use std::process;
use clap::Parser;
use Conversion::Cli;



fn main() {
    let cli = Cli::parse();
    println!("a: {:?}", cli);
    if let Err(e) = Conversion::run(cli) {
        println!("Problem err: {e}");
        process::exit(1);
    }

}


