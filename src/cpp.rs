

pub const DEFAULT_MAIN: &str = 
r#"
#include <iostream>
#include <string>
#include "dummy.hpp"

int main(int argc, char **argv) {

    dummy_hello("C++");
    std::cout << "You have successfully been created using cargo-newcpp" << std::endl;
}
"#;
