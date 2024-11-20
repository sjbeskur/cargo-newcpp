use std::process::Command;
use std::path::Path;

use cargo_newcpp::command_helper::dump_command;
use cli::BuildContext;
mod cli;

fn main() {

    println!("gtest...");
    let cfg = cli::parse_args();

    let path = match cfg.context{
        BuildContext::Debug(d) => d,
        BuildContext::Release(r) => r,
    };
    
    if Path::new(path).exists(){
        run_ctest(path);
    }
    color_print::ceprint!(" Path does not exist: {}", path);
}


fn run_ctest(target_dir: &str){
    let mut cmd = Command::new("ctest");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

