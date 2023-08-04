

fn main() {
    println!("TODO:// Implement the google test subcommand.");
}

const GTEST_DEFAULT: &str = 
"# Enable testing
enable_testing()

# Add FetchContent module
include(FetchContent)

# Define a new target for Google Test
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/heads/master.zip # URL to the Google Test repository
)

# Fetch Google Test and include it in the build
FetchContent_MakeAvailable(googletest)

# Add the Google Test include directories to your project
include_directories(${googletest_SOURCE_DIR}/googletest/include)
include_directories(${googletest_SOURCE_DIR}/googlemock/include)
";