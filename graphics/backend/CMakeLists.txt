cmake_minimum_required(VERSION 3.15)
project(GraphicsBackend)

set(SDL_STATIC ON CACHE BOOL "Use Static SDL" FORCE)
add_subdirectory(deps/sdl)

find_package(OpenGL REQUIRED)

add_library(backend SHARED window.cpp)

target_link_libraries(backend PUBLIC SDL3-static ${OPENGL_LIBRARY})
target_compile_definitions(backend PRIVATE INTERFACE_EXPORT)

install(TARGETS backend DESTINATION .)