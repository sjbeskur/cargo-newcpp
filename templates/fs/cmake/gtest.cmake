# Include Google Test using FetchContent
message("Fetching Google Test")
include(FetchContent)
FetchContent_Declare(
    googletest
    URL https://github.com/google/googletest/archive/refs/heads/master.zip # URL to Google Test repository
)
FetchContent_MakeAvailable(googletest)