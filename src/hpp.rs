pub const DEFAULT_HEADER: &str = 
r#"
#ifndef DUMMY_H
#define DUMMY_H

#include <iostream>

void dummy_hello(const std::string& message) {
    std::cout << "Hello " << message << std::endl;
}

#endif // DUMMY_H
"#;
