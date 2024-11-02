use std::path::Path;
use std::process::Command;

use cargo_newcpp::command_helper::dump_command;

fn main() {

    if !Path::new("./CMakeLists.txt").exists(){
        let curr_dir = std::env::current_dir().unwrap().to_string_lossy().to_string();
        println!("error: could not find `CMakeLists.txt` in `{}/CMakeLists.txt` or any parent directory",curr_dir);        
    }
    run_cmake("target/");
    run_ninja("target/");
}

// happy path 
fn run_cmake(target_dir: &str){
    let mut cmd = Command::new("cmake");
    cmd.arg("-B")
        .arg(target_dir)
        .arg("-S")
        .arg(".")
        .arg("-GNinja");

    dump_command(&mut cmd);
}

fn run_ninja(target_dir: &str){
    let mut cmd = Command::new("ninja");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

#[allow(dead_code)]
fn run_make(target_dir: &str){
    let mut cmd = Command::new("make");
    cmd.arg("-j`nproc`");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

