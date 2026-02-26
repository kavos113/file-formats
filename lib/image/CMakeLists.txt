cmake_minimum_required(VERSION 3.28)
project(libimage)

set(CMAKE_CXX_STANDARD 20)

add_library(libimage
        img/image.h
        img/format/ppm_parser.cpp
        img/format/ppm_parser.h
)

add_executable(image_ppm examples/image_ppm.cpp)
target_link_libraries(image_ppm PRIVATE libimage)