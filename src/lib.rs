use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod cpp_scafolding;
mod gitignore;
mod cmake;
mod readme;
pub mod command_helper;

pub use cpp_scafolding::*;
pub use std::error::Error;

pub fn make_project_dir(project_dir: &str) -> std::io::Result<()> {
    fs::create_dir(project_dir)?;
    Ok(())
}

pub fn make_defaults(project_dir: &str) -> Result<(),Box<dyn Error>> {

    make_default_files(project_dir, FileTypes::Main )?;
    make_default_files(project_dir, FileTypes::Header )?;
    make_default_files(project_dir, FileTypes::GitIgnore)?;
    
    let exe_template = include_str!("../templates/CMakeLists_exe.in");
    let cmake_template = cmake::get_cmake(project_dir, &exe_template)?;
    make_default_files(project_dir, FileTypes::Cmake(&cmake_template) )?;

    let template_readme = include_str!("../templates/README.md.in");
    let readme_template = readme::get_readme(project_dir, template_readme)?;
    make_default_files(project_dir, FileTypes::ReadMe(&readme_template) )?;

    make_default_files(project_dir, FileTypes::UnitTestExample )?;
    make_default_files(project_dir, FileTypes::CmakeTest )?;

    Ok(())
}

fn make_default_files(project_dir: &str, filetype: FileTypes  ) -> std::io::Result<()> {
    match filetype {
        FileTypes::Main => {            
            let mut file = File::create(project_dir.to_owned() + "/src/main.cpp")?;
            let template = include_str!("../templates/src/main.cpp.in");
            file.write_all(template.as_bytes())?;
        
        }
        FileTypes::Header => {
            let mut file = File::create(project_dir.to_owned() + "/include/dummy.hpp")?;
            let template = include_str!("../templates/include/dummy.hpp.in");
            file.write_all(template.as_bytes())?;
        
        }
        FileTypes::Cmake(value) => {
            let mut file = File::create(project_dir.to_owned() + "/CMakeLists.txt")?;
            file.write_all(value.as_bytes())?;
        
        }
        FileTypes::CmakeTest => {
            let mut file = File::create(project_dir.to_owned() + "/tests/CMakeLists.txt")?;
            let template = include_str!("../templates/test/CMakeGTest.make.in");
            file.write_all(template.as_bytes())?;        
            //file.write_all(value.as_bytes())?;        
        }
        
        FileTypes::UnitTestExample => {
            let mut file = File::create(project_dir.to_owned() + "/tests/example_test.cpp")?;
            let template = include_str!("../templates/test/example_test.cpp.in");
            file.write_all(template.as_bytes())?;        
        }
        FileTypes::GitIgnore => {
            let mut file = File::create(project_dir.to_owned() + "/.gitignore")?;
            let template = include_str!("../templates/gitignore.in");
            file.write_all(template.as_bytes())?;        
        }
        FileTypes::ReadMe(value) => {
            let mut file = File::create(project_dir.to_owned() + "/README.md")?;
            file.write_all(value.as_bytes())?;        
        }
        
    }

    Ok(())
}

pub enum FileTypes<'a>{
    Main,
    Header,
    Cmake(&'a str),
    CmakeTest,
    GitIgnore,
    UnitTestExample,
    ReadMe(&'a str),
}

