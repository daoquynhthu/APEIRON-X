cmake_minimum_required(VERSION 3.20)
project(apeiron-solver VERSION 0.1.0 LANGUAGES CXX CUDA)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CUDA_STANDARD 20)
set(CMAKE_CUDA_STANDARD_REQUIRED ON)

# Find CUDA
find_package(CUDA REQUIRED)
enable_language(CUDA)

# Find Python
find_package(Python3 COMPONENTS Interpreter Development NumPy REQUIRED)

# Find pybind11
find_package(pybind11 REQUIRED)

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CUDA_INCLUDE_DIRS}
    ${Python3_INCLUDE_DIRS}
)

# Source files
set(SOURCES
    src/tensor_engine/tensor_engine.cpp
    src/tensor_engine/mps.cpp
    src/tensor_engine/peps.cpp
    src/tensor_engine/tebd.cpp
    src/tensor_engine/tdvp.cpp
    src/topology_engine/cohomology.cpp
    src/topology_engine/betti.cpp
    src/transfinite_engine/transfinite.cpp
)

# CUDA source files
set(CUDA_SOURCES
    src/gpu_kernels/tensor_ops.cu
    src/gpu_kernels/evolution.cu
    src/gpu_kernels/truncation.cu
)

# Create library
add_library(apeiron_solver SHARED ${SOURCES} ${CUDA_SOURCES})

# Link libraries
target_link_libraries(apeiron_solver
    PRIVATE
    ${CUDA_LIBRARIES}
    ${CUDA_CUDART_LIBRARY}
    ${Python3_LIBRARIES}
    pybind11::module
)

# Python bindings
pybind11_add_module(apeiron_solver_py
    bindings/python_bindings.cpp
)

target_link_libraries(apeiron_solver_py PRIVATE apeiron_solver)

# Install
install(TARGETS apeiron_solver apeiron_solver_py
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)

# Tests
enable_testing()
add_subdirectory(tests)

