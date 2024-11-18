use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// #[command(propagate_version = true)]
// struct Cli {
//     #[command(subcommand)]
//     command: Option<Commands>,
// }


#[derive(Parser, Debug)]
#[command(bin_name = "cargo gtest", about = "A basic Cargo plugin example.")]
pub struct Cli {
    // #[command(subcommand)]
    // command: Option<Command>,

    #[arg(long, conflicts_with = "debug", default_value_t=false)]
    pub release: bool,

    #[arg(long, conflicts_with = "release",  default_value_t=true)]
    pub debug: bool,

}


/// A simple Cargo plugin example
// #[derive(Parser, Debug)]
// #[command(bin_name = "cargo gtest", about = "A basic Cargo plugin example.")]
// struct Cli {
//     /// A greeting to display
//     #[arg(short, long, default_value = "Hello")]
//     greeting: String,

//     /// Names to greet
//     #[arg()]
//     names: Vec<String>,
// }


    pub fn parse_args() -> bool {
        let args: Vec<String> = std::env::args().skip(1).collect();        
        let cli = Cli::parse_from(args);

        // // Print the greeting for each name
        // for name in cli.names {
        //     println!("{} {}!", cli.greeting, name);
        // }
        if cli.debug{
            println!("testing debug");
        }
        if cli.release{
            println!("testing release");
        }
        cli.debug

    }
