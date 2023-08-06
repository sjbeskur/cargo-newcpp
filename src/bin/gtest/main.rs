use std::process::Command;

use cargo_newcpp::command_helper::dump_command;

fn main() {

    run_ctest("target/");
}


fn run_ctest(target_dir: &str){
    let mut cmd = Command::new("ctest");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

