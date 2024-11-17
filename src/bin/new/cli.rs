pub struct Config{
    pub project_name: String,
    pub project_type: ProjectType,
}

#[derive(Debug, PartialEq)]
pub enum ProjectType{
    Lib,
    Exe
}

// total hack way to do this
pub fn parse_args() -> Config {
    let args: Vec<String> = std::env::args().collect();

    let mut proj_type = ProjectType::Exe;
    let islib = args.get(3).is_some_and(|v| v == "--lib" );
    if islib{ 
        println!("building new lib");
        proj_type = ProjectType::Lib;
    }
    if args.len() < 3 {
        println!("Invalid args.\n\tUsage: cargo newcpp <your_project_name> [--lib ]");
        std::process::exit(1);
    }

    Config { 
        project_name:  args[2].to_owned(),
        project_type: proj_type

    }
}