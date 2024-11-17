use std::process::Command;

use cargo_newcpp::*;
use cli::ProjectType;
use color_print::*;
mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = cli::parse_args();
    let islib = args.project_type == ProjectType::Lib;

    let proj = if islib {"library"} else{"application"};

    cprintln!("   <green,bold>Creating</green,bold> binary C++ ({}) `{}` package", proj, &args.project_name);
    let scaffold = CPPProjectScafolding::new(&args.project_name, islib);
    scaffold.generate_project_scafolding()?;
    run_git_init(&args.project_name);

    Ok(())
}

// happy path 
fn run_git_init(target_dir: &str){
    let mut cmd = Command::new("git");
    cmd.current_dir(target_dir);
    cmd.arg("init");
    command_helper::run_command(&mut cmd); 
    //command_helper::dump_command(cmd);
}
