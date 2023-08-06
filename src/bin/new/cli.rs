pub struct Config{
    pub project_name: String,
}

// total hack way to do this
pub fn parse_args() -> Config {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Invalid args.\n\tUsage: cargo newcpp <your_project_name>");
        std::process::exit(1);
    }

    Config { 
        project_name:  args[2].to_owned(),
    }
}