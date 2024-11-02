# cargo newcpp

This tool extends [Cargo](http://doc.crates.io/) to allow for the creation of new C++ projects in the spirit of `cargo new <project>`

Ensure that you have a fairly recent version of rust/cargo installed. On Ubuntu you might also want to install `cmake` and `gcc` or `g++` so that you can actually build project.

## Recommended prereqs 
If you'd like to kick the tires on the build feature (cargo buildcpp) you will want the following installed
```console,ignore
sudo apt install gcc, g++, ninja-build, build-essential, cmake
```


```console,ignore
cargo install cargo-newcpp
```

## Create a new project

Create a new CPP project with the following command.   
```console,ignore
cargo newcpp <your_project_name>
```

This will output to the following default project scaffolding.

![scaffolding](assets/project-files.png)

## Building the CPP project with cargo
Once you've create the project, you can use the following example to build the project.
```console,ignore
cd <your_project_name> 
cargo buildcpp
```
