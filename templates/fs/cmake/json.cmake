include(FetchContent)

# Try to find nlohmann/json on the system first
find_package(nlohmann_json QUIET)

if (NOT nlohmann_json_FOUND)
   message(STATUS "nlohmann/json not found, fetching using FetchContent.")
   # Fetch nlohmann/json
   FetchContent_Declare(
       nlohmann_json
       GIT_REPOSITORY https://github.com/nlohmann/json.git
       GIT_TAG        v3.11.3  # latest nlohmann (10-Dec-2024)
   )
   FetchContent_MakeAvailable(nlohmann_json)

endif()

# Link nlohmann_json to your target
#target_link_libraries(${PROJECT_NAME} nlohmann_json::nlohmann_json)