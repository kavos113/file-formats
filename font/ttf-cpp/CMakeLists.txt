cmake_minimum_required(VERSION 4.1)
project(fft_cpp)

set(CMAKE_CXX_STANDARD 20)

add_executable(fft_cpp main.cpp
        ttf/types.h)

target_include_directories(fft_cpp PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})