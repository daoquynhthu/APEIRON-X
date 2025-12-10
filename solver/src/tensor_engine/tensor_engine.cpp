#include "apeiron_solver/tensor_engine.h"
#include <stdexcept>

namespace apeiron::solver::tensor {

TensorEvolutionEngine::TensorEvolutionEngine() {
    // TODO: Initialize engine
}

void TensorEvolutionEngine::evolve_tebd(MPS& mps, double dt, size_t num_steps,
                                         const std::vector<Tensor>& hamiltonian) {
    // TODO: Implement TEBD evolution
    throw std::runtime_error("Not implemented");
}

void TensorEvolutionEngine::evolve_tdvp(MPS& mps, double dt, size_t num_steps,
                                         const std::vector<Tensor>& hamiltonian) {
    // TODO: Implement TDVP evolution
    throw std::runtime_error("Not implemented");
}

void TensorEvolutionEngine::set_gpu_devices(const std::vector<int>& gpu_ids) {
    gpu_devices_ = gpu_ids;
}

} // namespace apeiron::solver::tensor

