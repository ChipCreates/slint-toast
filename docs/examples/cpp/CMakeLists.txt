cmake_minimum_required(VERSION 3.21)
project(toast-example LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)

# Fetch Slint from GitHub. Pin to the release/1 branch for stability.
# Replace GIT_TAG with a specific version tag (e.g. v1.14.0) for reproducible builds.
include(FetchContent)
FetchContent_Declare(
    Slint
    GIT_REPOSITORY https://github.com/slint-ui/slint.git
    GIT_TAG release/1
    SOURCE_SUBDIR api/cpp
)
FetchContent_MakeAvailable(Slint)

add_executable(toast-example main.cpp)

# Compile the Slint UI file and link it to the target.
# slint_target_sources sets up include paths for the generated header automatically.
slint_target_sources(toast-example ui/app.slint)

target_link_libraries(toast-example PRIVATE Slint::Slint)
