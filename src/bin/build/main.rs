use std::path::Path;
use std::process::Command;

use cargo_newcpp::command_helper::dump_command;
mod cli;

fn main() {

    let args = cli::parse_args();

    let build_type =  match args.release{
        true => "Release",
        _ => "Debug",
    };

    if !Path::new("./CMakeLists.txt").exists(){
        let curr_dir = std::env::current_dir().unwrap().to_string_lossy().to_string();
        println!("error: could not find `CMakeLists.txt` in `{}/CMakeLists.txt` or any parent directory",curr_dir);        
    }
    let target_dir = format!("target/{build_type}/").to_lowercase();
    run_cmake(&target_dir, build_type);
    run_ninja(&target_dir);
}

// happy path 
fn run_cmake(target_dir: &str, build_type: &str){
    let mut cmd = Command::new("cmake");
    println!("INFO: creating build tree with default generator (ninja):");
    println!("  cmake -B {target_dir} -S . -G Ninja -D CMAKE_BUILD_TYPE={build_type}");        
    cmd.arg("-B")
        .arg(target_dir)
        .arg("-S")
        .arg(".")
        .arg("-GNinja")
        .arg(format!("-DCMAKE_BUILD_TYPE={}",build_type));

    dump_command(&mut cmd);
}

fn run_ninja(target_dir: &str){
    //let mut cmd = Command::new("ninja");
    //cmd.current_dir(target_dir);
    println!("INFO: executing command:");
    println!("  cmake --build {target_dir}");        
    let mut cmd = Command::new("cmake");
    cmd.arg("--build")
        .arg(target_dir);

    dump_command(&mut cmd);
}

#[allow(dead_code)]
fn run_make(target_dir: &str){
    let mut cmd = Command::new("cmake");
    cmd.arg("--build");
    cmd.arg("-j`nproc`");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

