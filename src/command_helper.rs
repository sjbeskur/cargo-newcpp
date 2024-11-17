use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn dump_command(cmd: &mut Command){
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

pub fn run_command(cmd: &mut Command){

    cmd.stdout(Stdio::piped());  
    cmd.stderr(Stdio::piped());  
    let mut child = cmd.spawn().unwrap();  
    // let stdout = child.stdout.take().expect("Failed to get stdout");
    // let sterr = child.stderr.take().expect("Failed to get stderr");

    // Wait for the command to finish and check if it was successful
    let status = child.wait().unwrap();
    if !status.success() {
        println!("Command failed with exit code: {:?}", status.code());
    }
}

/*
    Ok(_) => println!("Was spawned :)"),
    Err(e) => {
        if let NotFound = e.kind() {
            println!("`rustc` was not found! Check your PATH!")
        } else {
            println!("Some strange error occurred :(");
        }
    }, 
*/