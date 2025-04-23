# CMakeLists.txt
cmake_minimum_required(VERSION 3.10)
project(bin_tree)

set(CMAKE_C_STANDARD 11)  # Specify C standard

# Create directories if they don't exist
file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/obj)
file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

include_directories(include)  # Add include directory

file(GLOB_RECURSE SOURCES "src/*.c") # Gather source files

add_executable(${PROJECT_NAME} ${SOURCES})

set_target_properties(${PROJECT_NAME} PROPERTIES
    COMPILE_FLAGS "-Wall -Wextra -O2" # Compiler flags
    OBJECT_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/obj"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)
