use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod cpp_scafolding;
mod cmake;
mod readme;
pub mod command_helper;

pub use cpp_scafolding::*;
pub use std::error::Error;

pub fn make_project_dir(project_dir: &str) -> std::io::Result<()> {
    fs::create_dir(project_dir)?;
    Ok(())
}

pub fn make_defaults(project_dir: &str, is_library: bool) -> Result<(),Box<dyn Error>> {

    if is_library{
        make_default_files(project_dir, FileTypes::Library )?;

        let lib_template = include_str!("../templates/fs/CMakeLists.txt.lib.in");
        let cmake_template = cmake::get_cmake(project_dir, lib_template)?;
        make_default_files(project_dir, FileTypes::Cmake(&cmake_template) )?;
    
    }else{
        make_default_files(project_dir, FileTypes::Main )?;
    
        let exe_template = include_str!("../templates/fs/CMakeLists.txt.exe.in");
        let cmake_template = cmake::get_cmake(project_dir, exe_template)?;
        make_default_files(project_dir, FileTypes::Cmake(&cmake_template) )?;
    }

    make_default_files(project_dir, FileTypes::Header )?;
    make_default_files(project_dir, FileTypes::GitIgnore)?;
    
    let template_readme = include_str!("../templates/fs/README.md.in");
    let readme_template = readme::get_readme(project_dir, template_readme)?;
    make_default_files(project_dir, FileTypes::ReadMe(&readme_template) )?;

    make_default_files(project_dir, FileTypes::UnitTestExample )?;
    make_default_files(project_dir, FileTypes::CmakeTest )?;

    make_default_files(project_dir, FileTypes::CmakeModule )?;


    Ok(())
}

fn make_default_files(project_dir: &str, filetype: FileTypes  ) -> std::io::Result<()> {
    match filetype {
        FileTypes::Main => {            
            let mut file = File::create(project_dir.to_owned() + "/src/main.cpp")?;
            let template = include_str!("../templates/fs/src/main.cpp.in");
            file.write_all(template.as_bytes())?;
        
        }

        FileTypes::Library => {            
            let mut file = File::create(project_dir.to_owned() + "/src/mylibrary.cpp")?;
            let template = include_str!("../templates/fs/src/mylibrary.cpp.in");
            file.write_all(template.as_bytes())?;
        
        }        
        FileTypes::Header => {
            let mut file = File::create(project_dir.to_owned() + "/include/dummy.hpp")?;
            let template = include_str!("../templates/fs/include/dummy.hpp.in");
            file.write_all(template.as_bytes())?;
        
        }
        FileTypes::Cmake(value) => {
            let mut file = File::create(project_dir.to_owned() + "/CMakeLists.txt")?;
            file.write_all(value.as_bytes())?;
        
        }
        FileTypes::CmakeTest => {
            let mut file = File::create(project_dir.to_owned() + "/tests/CMakeLists.txt")?;
            let template = include_str!("../templates/fs/test/CMakeGTest.make.in");
            file.write_all(template.as_bytes())?;        
            //file.write_all(value.as_bytes())?;        
        }
        
        FileTypes::UnitTestExample => {
            let mut file = File::create(project_dir.to_owned() + "/tests/example_test.cpp")?;
            let template = include_str!("../templates/fs/test/example_test.cpp.in");
            file.write_all(template.as_bytes())?;        
        }
        FileTypes::GitIgnore => {
            let mut file = File::create(project_dir.to_owned() + "/.gitignore")?;
            let template = include_str!("../templates/fs/gitignore.in");
            file.write_all(template.as_bytes())?;        
        }
        FileTypes::ReadMe(value) => {
            let mut file = File::create(project_dir.to_owned() + "/README.md")?;
            file.write_all(value.as_bytes())?;        
        },
        FileTypes::CmakeModule => {
            let files = vec![
                "cmake/eigen.cmake",
                "cmake/fftw.cmake",
                "cmake/json.cmake",
                "cmake/gtest.cmake",
                "cmake/opencv.cmake",
            ];
            let templates = vec![
                include_str!("../templates/fs/cmake/eigen.cmake"),
                include_str!("../templates/fs/cmake/fftw.cmake"),  
                include_str!("../templates/fs/cmake/json.cmake"),
                include_str!("../templates/fs/cmake/gtest.cmake"),
                include_str!("../templates/fs/cmake/opencv.cmake"),
            ];

            files.iter().zip(templates.iter()).for_each(|(file, template)| {
                let mut file = File::create(project_dir.to_owned() + "/" + file).expect(&format!("Failed to create file: {:?}", file));
                file.write_all(template.as_bytes()).expect(&format!("Failed to write to file: {:?}", file));
            });
        }
        
    }

    Ok(())
}

pub enum FileTypes<'a>{
    Main,
    Library,
    Header,
    Cmake(&'a str),
    CmakeModule,
    CmakeTest,
    GitIgnore,
    UnitTestExample,
    ReadMe(&'a str),
}

