cmake_minimum_required(VERSION 3.20)
project(with_lua LANGUAGES C)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK REQUIRED IMPORTED_TARGET "gtk4")

find_package(Lua REQUIRED)
# pkg_check_modules(LUA REQUIRED IMPORTED_TARGET "lua5.4")
# include_directories(/usr/include/lua5.4)
# include_directories(/usr/include $LUA_INCLUDE_DIR})
# include_directories(/usr/local/include/lua5.4)
include_directories(/usr/local/include $LUA_INCLUDE_DIR})

add_executable(with_lua src/main.c)
target_link_libraries(with_lua PRIVATE PkgConfig::GTK)
