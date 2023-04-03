cmake_minimum_required(VERSION 3.15)
project(LinkCWithRust)

add_library(CLib STATIC CLib.cpp)

install(TARGETS CLib DESTINATION .)