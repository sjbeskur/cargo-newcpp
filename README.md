# cargo newcpp

This tool extends [Cargo](http://doc.crates.io/) to allow for the creation of new C++ projects in the spirit of `cargo new <project>`

```console,ignore
cargo install cargo-newcpp
```

## Required Prereqs 
Ensure that you have a fairly recent version of Rust/Cargo installed. 

In order to build projects using ```cargo buildcpp``` you will also want to install recent versions of standard C++ build tools:

```console,ignore
sudo apt install gcc, g++, ninja-build, build-essential, cmake
```

## Create a new project
To create a new C++ project:
```console,ignore
cargo newcpp <your_project_name> [--lib]
```

This will output to the following default project scaffolding under the <project_name> folder.

![scaffolding](assets/project-files.png)

If you have ```git``` installed this will also initialize your new project as a git repo.


## Building the CPP project with cargo
Once you've created the project, you can build it using the command below:
```console,ignore
cd <your_project_name> 
cargo buildcpp [--debug | --release]
```

By default, ```buildcpp``` will place cmake buildtree artifacts into the **target/debug** folder.  Specifying the **--release** flag will generate the buildtree artifacts in **target/release**


## Cleaning the CPP project with cargo-cleancpp
This is functionally equivilant to "rm -rf target/".  I struggled with whether or not I should do something more idiomatic to modern cmake (e.g. ```cmake --build <target> -t clean``` or ```cmake --fresh ...```) but these did not feel right and chose to default to what ```cargo clean`` does instead.

```console,ignore
cargo cleancpp 
```

