use cargo_newcpp::*;

use clap::Parser;
mod cli;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2{ return; }    

    args.drain(0..2);
    for (i,arg) in args.iter().enumerate(){
        println!("-->arg{} = {}",i, arg);
    }

    let args = cli::Config::parse();

    let scaffold = CPPProjectScafolding::new(args.project_name);
    scaffold.generate_project_scafolding();

}
