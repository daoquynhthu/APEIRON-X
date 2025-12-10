#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <pybind11/numpy.h>
#include "apeiron_solver/tensor_engine.h"
#include "apeiron_solver/topology_engine.h"
#include "apeiron_solver/transfinite_engine.h"

namespace py = pybind11;

PYBIND11_MODULE(apeiron_solver_py, m) {
    m.doc() = "APEIRON-X Solver Python Bindings";
    
    // Tensor Engine
    py::module tensor_module = m.def_submodule("tensor", "Tensor Evolution Engine");
    
    py::class_<apeiron::solver::tensor::MPS>(tensor_module, "MPS")
        .def(py::init<size_t, size_t>())
        .def("evolve_tebd", &apeiron::solver::tensor::MPS::evolve_tebd)
        .def("evolve_tdvp", &apeiron::solver::tensor::MPS::evolve_tdvp)
        .def("truncate", &apeiron::solver::tensor::MPS::truncate)
        .def("get_tensors", &apeiron::solver::tensor::MPS::get_tensors)
        .def("get_bond_dim", &apeiron::solver::tensor::MPS::get_bond_dim);
    
    py::class_<apeiron::solver::tensor::TensorEvolutionEngine>(tensor_module, "TensorEvolutionEngine")
        .def(py::init<>())
        .def("evolve_tebd", &apeiron::solver::tensor::TensorEvolutionEngine::evolve_tebd)
        .def("evolve_tdvp", &apeiron::solver::tensor::TensorEvolutionEngine::evolve_tdvp)
        .def("set_gpu_devices", &apeiron::solver::tensor::TensorEvolutionEngine::set_gpu_devices);
    
    // Topology Engine
    py::module topology_module = m.def_submodule("topology", "Topology Engine");
    
    py::class_<apeiron::solver::topology::CohomologyAnalyzer>(topology_module, "CohomologyAnalyzer")
        .def(py::init<>())
        .def("compute_betti_numbers", &apeiron::solver::topology::CohomologyAnalyzer::compute_betti_numbers)
        .def("identify_generators", &apeiron::solver::topology::CohomologyAnalyzer::identify_generators)
        .def("detect_surgery_trigger", &apeiron::solver::topology::CohomologyAnalyzer::detect_surgery_trigger);
    
    // Transfinite Engine
    py::module transfinite_module = m.def_submodule("transfinite", "Transfinite Recursor");
    
    py::class_<apeiron::solver::transfinite::TransfiniteRecursor>(transfinite_module, "TransfiniteRecursor")
        .def(py::init<>())
        .def("expand_ordinal_layer", &apeiron::solver::transfinite::TransfiniteRecursor::expand_ordinal_layer)
        .def("reset_evolution_time", &apeiron::solver::transfinite::TransfiniteRecursor::reset_evolution_time)
        .def("detect_fixed_point", &apeiron::solver::transfinite::TransfiniteRecursor::detect_fixed_point)
        .def("get_evolution_time", &apeiron::solver::transfinite::TransfiniteRecursor::get_evolution_time);
}

