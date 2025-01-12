cmake_minimum_required(VERSION 3.30)
project(my_space_invaders_emulator)

set(CMAKE_CXX_STANDARD 20)

add_executable(my_space_invaders_emulator main.cpp
        constants.h
        cpu.cpp
        cpu.h
        cpu/flags.h
        cpu/registers.h
        cpu/stack.cpp
        cpu/stack.h
)
