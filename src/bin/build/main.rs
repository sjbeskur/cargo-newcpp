use std::path::Path;
use std::process::Command;

use cargo_newcpp::command_helper::dump_command;

fn main() {

    if !Path::new("./CMakeLists.txt").exists(){
        let curr_dir = std::env::current_dir().unwrap().to_string_lossy().to_string();
        println!("error: could not find `CMakeLists.txt` in `{}/{}` or any parent directory",curr_dir, "CMakeLists.txt");        
    }
    run_cmake("target/");
    run_make("target/");
}

// happy path 
fn run_cmake(target_dir: &str){
    let mut cmd = Command::new("cmake");
    cmd.arg("-B")
        .arg(target_dir)
        .arg("-S")
        .arg(".");

    dump_command(&mut cmd);
}

fn run_make(target_dir: &str){
    let mut cmd = Command::new("make");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

