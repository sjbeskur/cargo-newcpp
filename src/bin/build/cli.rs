use clap::Parser;

/// Define the build types available.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum CmakeBuildType {
    Release,
    Debug,
}


/// CLI structure with clap for parsing arguments.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    // Optionally specify the build type (Release or Debug).
    //#[arg(short, long)]
    #[clap(long, conflicts_with = "debug", default_value_t=false)]
    pub release: bool,

    #[clap(long, conflicts_with = "release",  default_value_t=true)]
    pub debug: bool,

}

#[allow(dead_code)]
// total hack way to do this
pub fn parse_args() -> Cli {
    // Parse CLI arguments.
    let args = Cli::parse();

    // Output the configuration.
    println!("{:#?}", args);
    args
    
}