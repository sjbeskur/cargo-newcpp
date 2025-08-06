use std::process::Command;
use std::path::Path;

use cargo_newcpp::command_helper::dump_command;
use cli::BuildContext;
mod cli;

fn main() {

    println!("Running GoogleTest units with cargo-gtest: ");
    let cfg = cli::parse_args();

    let path = match cfg.context{
        BuildContext::Debug(d) => d,
        BuildContext::Release(r) => r,
    };
    
    let test_path = Path::new(path).join("tests");
    if test_path.exists(){
        run_ctest(test_path.as_os_str().to_str().unwrap());
    }else{
        color_print::ceprint!(" Path does not exist: {:?}", test_path);
    }
}


fn run_ctest(target_dir: &str){
    let mut cmd = Command::new("ctest");
    cmd.current_dir(target_dir);

    dump_command(&mut cmd);
}

