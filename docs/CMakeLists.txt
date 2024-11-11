For the CMakeLists.txt in the root directory:


cmake_minimum_required(VERSION 3.10)

# Set your project name here
project( t1 VERSION 1.0 LANGUAGES CXX DESCRIPTION "Project description here" )

# Setup CMake configuration options here
option(BUILD_UNIT_TESTS "BUILD_UNIT_TESTS Builds the unit test executable" ON) # to switch off: -DBUILD_UNIT_TESTS=OFF

# Set the C++ standard to use (change to your preferred version)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})

message ("===================================================================")
message ("Configuring ${PROJECT_NAME} Main exsecutable")
message ("===================================================================\n")


# Add your source files here (replace "main.cpp" with your actual source files)
set(SOURCES
    src/main.cpp
)

# Create an executable target from the source files
add_executable(${PROJECT_NAME} ${SOURCES})

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
if (BUILD_UNIT_TESTS)
    add_subdirectory(tests)
    enable_testing()
endif()

# Optionally, set the output directory for the built binaries
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

message("### Configuration complete ###\n\n")

Add some printouts to show various configure stages

Remove setting global includes but rather target include directories for the compiler to find the includes

Add a CMake option to switch off building the unit tests if unecessary
For the tests CMakeLists.txt:

message ("===================================================================")
message ("Configuring ${PROJECT_NAME} Unit Tests")
message ("===================================================================\n")

# Include Google Test using FetchContent
message("Fetching Google Test")
include(FetchContent)
FetchContent_Declare(
    googletest
    URL https://github.com/google/googletest/archive/refs/heads/master.zip # URL to Google Test repository
)
FetchContent_MakeAvailable(googletest)

# Search for units tests
file(GLOB_RECURSE TEST_SOURCES *.cpp)
set(TEST_SRC_FILES ${TEST_SOURCES})

# Add headers, if any (replace "header.h" with your actual header files)
message("Current Source Dir:" ${CMAKE_CURRENT_SOURCE_DIR})
# Add the test executable
add_executable(unit_tests ${TEST_SRC_FILES})
# Set the test include directories
target_include_directories(unit_tests PUBLIC ../include)
# Link the test executable with the main library and Google Test
target_link_libraries(unit_tests PRIVATE  gtest_main)

# Add the test to CTest
include(CTest)
include(GoogleTest)
gtest_discover_tests(unit_tests)




Again remove global includes rather set the include directory(ies)

Make the test code files auto-discoverable rather than having to modify this file for every new test