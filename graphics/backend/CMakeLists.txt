cmake_minimum_required(VERSION 3.15)
project(GraphicsBackend)

set(SDL_STATIC ON CACHE BOOL "Use Static SDL" FORCE)
add_subdirectory(deps/sdl)

find_package(OpenGl REQUIRED)

add_library(backend STATIC window.cpp)

target_link_libraries(backend SDL3-static OpenGl)

install(TARGETS backend DESTINATION .)