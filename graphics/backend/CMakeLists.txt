cmake_minimum_required(VERSION 3.15)
project(GraphicsBackend)

add_library(backend STATIC window.cpp)

install(TARGETS backend DESTINATION .)