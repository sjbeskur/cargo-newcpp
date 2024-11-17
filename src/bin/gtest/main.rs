use std::process::Command;
use std::path::Path;

use cargo_newcpp::command_helper::dump_command;

fn main() {
    let target_dir = "target/debug";
    if Path::new(target_dir).exists(){
        run_ctest("target/debug");
    }
    color_print::ceprint!(" Path does not exist: {}", target_dir);
}


fn run_ctest(target_dir: &str){
    let mut cmd = Command::new("ctest");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

