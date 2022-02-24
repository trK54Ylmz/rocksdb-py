cmake_minimum_required(VERSION 3.10)

project(rocksdb_bench)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 -Wall")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

find_package(Rocksdb)

if((NOT ROCKSDB_FOUND))
  find_path(ROCKSDB_ROOT_DIR NAMES include/rocksdb/db.h)

  find_library(
    ROCKSDB_LIBRARIES
    NAMES rocksdb
    HINTS ${ROCKSDB_ROOT_DIR})

  find_path(
    ROCKSDB_INCLUDE_DIR
    NAMES rocksdb/db.h
    HINTS ${ROCKSDB_ROOT_DIR}/include)

  include(FindPackageHandleStandardArgs)

  find_package_handle_standard_args(Rocksdb DEFAULT_MSG ROCKSDB_LIBRARIES
                                    ROCKSDB_INCLUDE_DIR)

  mark_as_advanced(ROCKSDB_ROOT_DIR ROCKSDB_LIBRARIES ROCKSDB_INCLUDE_DIR)
endif()

file(GLOB SOURCES "src/*.h" "src/*.cpp")

message(STATUS "Library is ${ROCKSDB_LIBRARIES}")
message(STATUS "Includes is ${ROCKSDB_INCLUDE_DIR}")

include_directories(${ROCKSDB_INCLUDE_DIR})

add_executable(${PROJECT_NAME} ${SOURCES})
target_link_libraries(${PROJECT_NAME} ${ROCKSDB_LIBRARIES})
