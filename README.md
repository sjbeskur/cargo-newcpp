# cargo newcpp

This tool extends [Cargo](http://doc.crates.io/) to allow for the creation of new C++ projects in the spirit of `cargo new <project>`

Ensure that you have a fairly recent version of rust/cargo installed. On Ubuntu you might also want to install `cmake` and `gcc` or `g++` so that you can actually build project.

```console,ignore
cargo install cargo-newcpp
```

## Usage

```console,ignore
cargo newcpp <project_name>
```

![scaffolding](assets/project-files.png)

```console,ignore
cd <project_name> 
cmake -B build -S . && cd build && make
```
