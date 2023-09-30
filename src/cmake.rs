use handlebars::Handlebars;
use std::collections::BTreeMap;

pub fn get_cmake(project_name: &str) -> Result<String, Box<dyn std::error::Error>>{
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("cmake_template", DEFAULT_CMAKE)?;

    let mut data = BTreeMap::new();
    data.insert("project_name".to_string(), project_name.to_string());
    let rslt = handlebars.render("cmake_template", &data)?;
    return Ok(rslt);
}


pub const DEFAULT_CMAKE: &str = r#"
cmake_minimum_required(VERSION 3.10)

# Set your project name here
project( {{project_name}} )

# Set the C++ standard to use (change to your preferred version)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})

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
add_subdirectory(tests)
enable_testing()


# add_test(NAME MyTest COMMAND MyTestExecutable)

# Optionally, set the output directory for the built binaries
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

"#;


pub const DEFAULT_GTEST_CMAKE: &str = r#"# Enable testing

# Include Google Test using FetchContent

message("Fetching Google Test")

include(FetchContent)
FetchContent_Declare(
    googletest
    URL https://github.com/google/googletest/archive/refs/heads/master.zip # URL to Google Test repository
)
FetchContent_MakeAvailable(googletest)

# Add headers, if any (replace "header.h" with your actual header files)
message("Current Source Dir:" ${CMAKE_CURRENT_SOURCE_DIR})
include_directories(../include)
set(HEADERS
    ../include/dummy.hpp
)
# Add the test executable
add_executable(unit_tests ${HEADERS} example_test.cpp)

# Link the test executable with the main library and Google Test
target_link_libraries(unit_tests PRIVATE  gtest_main)

# Add the test to CTest
include(CTest)
include(GoogleTest)
gtest_discover_tests(unit_tests)
"#;