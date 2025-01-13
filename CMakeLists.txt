cmake_minimum_required(VERSION 3.30)
project(my_space_invaders_emulator)

set(CMAKE_CXX_STANDARD 23)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)

find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIR})

add_executable(my_space_invaders_emulator main.cpp
        constants.h
        cpu.cpp
        cpu.h
        cpu/flags.h
        cpu/registers.h
        cpu/stack.cpp
        cpu/stack.h
        emulator.cpp
        emulator.h
)

target_link_libraries(my_space_invaders_emulator ${SDL2_LIBRARY})
