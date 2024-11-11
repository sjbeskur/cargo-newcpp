//use clap::Parser;

/// CLI structure with clap for parsing arguments.
// #[derive(Parser, Debug)]
// #[clap(author, version, about)]
// //#[command(author, version, about, long_about = None)]
// pub struct Cli {
//     // Optionally specify the build type (Release or Debug).
//     //#[arg(short, long)]
//     #[clap(long, conflicts_with = "debug", default_value_t=false)]
//     pub release: bool,

//     #[clap(long, conflicts_with = "release",  default_value_t=true)]
//     pub debug: bool,

// }


pub fn parse_args() -> bool {
    // // Parse CLI arguments.
    // let args = Cli::parse();

    // // Output the configuration.
    // println!("{:#?}", args);
    // args

    let args: Vec<String> = std::env::args().collect();
    // println!("... args.0 = {}", args[0]);
    // println!("... args.1 = {}", args[1]);
    // println!("... args.2 = {}", args[2]);
    // println!("... args.len() = {}", args.len());

    if args.len() < 2 || args.len() > 3{
        println!("Invalid args.\n\tUsage: cargo buildcpp [--debug ] | [--release]");
        std::process::exit(1);
    }

    let mut release = false;
    if args.len() == 3{
        match args[2].as_str(){
            "--release" =>  { release = true; }
            "--debug" => { release = false; }    
            _ => {
                println!("Invalid args.\n\tUsage: cargo buildcpp [--debug ] | [--release]");
                std::process::exit(1);
            }     
        }
    }

    release
    
}