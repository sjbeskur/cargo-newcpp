use handlebars::Handlebars;
use std::collections::BTreeMap;

pub fn get_cmake(project_name: &str) -> String{
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("cmake_template", DEFAULT_CMAKE);

    let mut data = BTreeMap::new();
    data.insert("project_name".to_string(), project_name.to_string());
    handlebars.render("cmake_template", &data).unwrap()
}


pub const DEFAULT_CMAKE: &str = r#"
cmake_minimum_required(VERSION 3.10)

# Set your project name here
project( {{project_name}} )

# Set the C++ standard to use (change to your preferred version)
set(CMAKE_CXX_STANDARD 17)

# Add your source files here (replace "main.cpp" with your actual source files)
set(SOURCES
    src/main.cpp
)

# Add headers, if any (replace "header.h" with your actual header files)
set(HEADERS
    include/dummy.hpp
)

# Create an executable target from the source files
add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

# Set the include directories (replace "include" with your actual include path)
target_include_directories(${PROJECT_NAME} PUBLIC
    include
)

# Set any additional compiler flags if needed
# For example:
# target_compile_options(${PROJECT_NAME} PRIVATE -Wall -Wextra)

# If you have any external libraries to link against, use the following format:
# find_package(YourLibrary REQUIRED)
# target_link_libraries(${PROJECT_NAME} PRIVATE YourLibrary::YourLibrary)

# Optionally, you can enable testing with Google Test
# add_subdirectory(tests)
# enable_testing()
# add_test(NAME MyTest COMMAND MyTestExecutable)

# Optionally, set the output directory for the built binaries
# set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

"#;