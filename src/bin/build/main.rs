use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn main() {

    if !Path::new("./CMakeLists.txt").exists(){
        let curr_dir = std::env::current_dir().unwrap().to_string_lossy().to_string();
        println!("error: could not find `CMakeLists.txt` in `{}/{}` or any parent directory",curr_dir, "CMakeLists.txt");        
    }
    run_cmake("target/");
    run_make("target/");
}

// happy path 
// let output = Command::new("cmake")
//     .arg("-B")
//     .arg("target/")
//     .arg("-S")
//     .arg(".")
//     .output()
//     .expect("error: could not find `cmake` in path" );
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

fn dump_command(cmd: &mut Command){
    cmd.stdout(Stdio::piped());  
    let mut child = cmd.spawn().unwrap();  
    let stdout = child.stdout.take().expect("Failed to get stdout");

    // Create a buffer reader to read the output line by line
    let reader = BufReader::new(stdout);

    // Read and print the output in real-time
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }    

    // Wait for the command to finish and check if it was successful
    let status = child.wait().unwrap();
    if !status.success() {
        println!("Command failed with exit code: {:?}", status.code());
    }

}
