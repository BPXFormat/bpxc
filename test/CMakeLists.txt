project(bpxc_test)
cmake_minimum_required(VERSION 3.1)

add_executable(bpxc_test main.c)

target_include_directories(bpxc_test PRIVATE ../include)
target_link_directories(bpxc_test PRIVATE ../)
target_link_libraries(bpxc_test PRIVATE bpxc)
