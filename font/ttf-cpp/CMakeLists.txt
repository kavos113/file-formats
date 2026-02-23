cmake_minimum_required(VERSION 4.1)
project(ttf_cpp)

set(CMAKE_CXX_STANDARD 20)

add_executable(ttf_cpp main.cpp
        ttf/types.h
        ttf/table_directory.h
        util/util.h
        tables/os2.h
        tables/os2.cpp)

target_include_directories(ttf_cpp PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})