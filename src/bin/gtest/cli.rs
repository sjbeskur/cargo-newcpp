use clap::{Parser, ArgGroup};

#[derive(Parser, Debug)]
#[command(bin_name = "cargo gtest", author, version, about = "A basic Cargo plugin example.", long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .args(&["release", "debug"])
        .required(false) // Set to true if you want one of them to be mandatory
))]
pub struct Cli {

    #[arg(long, conflicts_with = "debug", default_value_t=false)]
    pub release: bool,

    #[arg(long, conflicts_with = "release",  default_value_t=false)]
    pub debug: bool,

}


pub enum BuildContext<'a>{
    Debug(&'a str),
    Release(&'a str),
}

pub struct Config<'a>{
    pub context: BuildContext<'a>,
}

impl Config<'static>{
    pub fn new(is_release: bool) -> Config<'static>{
        let context = match is_release{
            true => BuildContext::Release("target/release"),
            false  => BuildContext::Debug("target/debug"),
        };
        Self{ context }
    }
}    

// ln -s $(pwd)/target/debug/cargo-gtest ~/.local/bin/
pub fn parse_args() -> Config<'static> {
    let args: Vec<String> = std::env::args().skip(1).collect();        
    let cli = Cli::parse_from(args);
    if cli.release && cli.debug {
        eprintln!("Error: Only one of --release or --debug can be set.");
        std::process::exit(1);
    }
    let is_release = cli.release;

    Config::new(is_release)
}
