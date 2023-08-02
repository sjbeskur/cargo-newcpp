use clap::Parser;

#[derive(Debug, Parser)]
#[command(author="Sam Beskur <sam.beskur@gmail.com>", version
, about="C++ Project scaffolding generator"
, long_about = "A cargo plugin for creating new C++ projects.")]
pub struct Config{
    pub project_name: String,

}