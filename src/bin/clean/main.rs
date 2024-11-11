use std::process::Command;

use cargo_newcpp::command_helper::dump_command;

fn main() {
    run_clean("target/");
}

// happy path 
fn run_clean(target_dir: &str){
    // perhaps consider:
    // cmake --build <target> -t clean 
    let mut cmd = Command::new("rm");
    cmd.arg("-rf")
        .arg(target_dir);

    dump_command(&mut cmd);
}
