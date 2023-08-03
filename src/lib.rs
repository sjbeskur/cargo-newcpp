use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod cpp_scafolding;
mod cpp;
mod hpp;
mod gitignore;
mod cmake;

pub use cpp_scafolding::*;
pub use std::error::Error;

pub fn make_project_dir(project_dir: &str) -> std::io::Result<()> {
    fs::create_dir(project_dir)?;
    Ok(())
}

pub fn make_defaults(project_dir: &str) -> Result<(),Box<dyn Error>> {

    make_default_files(project_dir, FileTypes::Main(cpp::DEFAULT_MAIN) )?;
    make_default_files(project_dir, FileTypes::Header(hpp::DEFAULT_HEADER) )?;
    make_default_files(project_dir, FileTypes::GitIgnore(gitignore::DEFAULT_GITIGNORE) )?;
    
    let cmake_template = cmake::get_cmake(project_dir)?;
    make_default_files(project_dir, FileTypes::Cmake(&cmake_template) )?;
    Ok(())
}

fn make_default_files(project_dir: &str, filetype: FileTypes  ) -> std::io::Result<()> {
    match filetype {
        FileTypes::Main(value) => {
            let mut file = File::create(project_dir.to_owned() + "/src/main.cpp")?;
            file.write_all(value.as_bytes())?;
        
        }
        FileTypes::Header(value) => {
            let mut file = File::create(project_dir.to_owned() + "/include/dummy.hpp")?;
            file.write_all(value.as_bytes())?;
        
        }
        FileTypes::Cmake(value) => {
            let mut file = File::create(project_dir.to_owned() + "/CMakeLists.txt")?;
            file.write_all(value.as_bytes())?;
        
        }
        FileTypes::GitIgnore(value) => {
            let mut file = File::create(project_dir.to_owned() + "/.gitignore")?;
            file.write_all(value.as_bytes())?;
        
        }
        
    }

    Ok(())
}

pub enum FileTypes<'a>{
    Main(&'a str),
    Header(&'a str),
    Cmake(&'a str),
    GitIgnore(&'a str),
//    ReadMe(&'a str),
}

