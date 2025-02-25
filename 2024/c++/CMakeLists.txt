cmake_minimum_required(VERSION 3.10)
project(my_cpp_project VERSION 1.0)

# Set C++ standard to C++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)  # Helps with LSP integration

# Apple M1 specific settings
if(APPLE AND CMAKE_SYSTEM_PROCESSOR STREQUAL "arm64")
    # Ensure we're using the correct architecture
    set(CMAKE_OSX_ARCHITECTURES "arm64")
    
    # Optional: Use Homebrew-installed libraries if needed
    # Many libraries are installed at /opt/homebrew for Apple Silicon
    include_directories(/opt/homebrew/include)
    link_directories(/opt/homebrew/lib)
endif()

# Add include directory
include_directories(${PROJECT_SOURCE_DIR}/include)

# Add executable
add_executable(${PROJECT_NAME} src/main.cpp)

# Add any libraries if needed
# target_link_libraries(${PROJECT_NAME} PRIVATE some_library)
