use cargo_newcpp::*;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut args = std::env::args().collect::<Vec<String>>();
    //if args.len() < 2{ return; }    
    //args.drain(0..2);

    let args = cli::parse_args();

    let scaffold = CPPProjectScafolding::new(args.project_name);
    scaffold.generate_project_scafolding()?;

    Ok(())
}
