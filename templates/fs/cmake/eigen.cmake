message(CHECK_START "Fetching Eigen3")
list(APPEND CMAKE_MESSAGE_INDENT "  ")

find_package(Eigen QUIET)
   
if (NOT Eigen_FOUND)
  message(STATUS "Eigen not found, attempting to fetch using FetchContent.")

  include(FetchContent)
    FetchContent_Declare(
      Eigen
      GIT_REPOSITORY https://gitlab.com/libeigen/eigen.git
      #GIT_TAG master
      GIT_TAG        3.4.0  # Or whatever Eigen version you want to use
      GIT_SHALLOW TRUE
      GIT_PROGRESS TRUE)

endif()


# note: To disable eigen tests,
# you should put this code in a add_subdirectory to avoid to change
# BUILD_TESTING for your own project too since variables are directory
# scoped
# set(BUILD_TESTING OFF) # need to figure out
set(EIGEN_BUILD_TESTING OFF)
set(EIGEN_MPL2_ONLY ON)
set(EIGEN_BUILD_PKGCONFIG OFF)
set(EIGEN_BUILD_DOC OFF)
FetchContent_MakeAvailable(Eigen)

list(POP_BACK CMAKE_MESSAGE_INDENT)
message(CHECK_PASS "fetched")

target_link_libraries(${PROJECT_NAME} PRIVATE Eigen3::Eigen)