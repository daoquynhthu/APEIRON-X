#pragma once

#include <vector>
#include <complex>
#include <memory>

namespace apeiron::solver::tensor {

using Complex = std::complex<double>;
using Tensor = std::vector<Complex>;

/// Matrix Product State (MPS)
class MPS {
public:
    MPS(size_t num_sites, size_t bond_dim);
    
    // Evolution methods
    void evolve_tebd(double dt, const std::vector<Tensor>& hamiltonian);
    void evolve_tdvp(double dt, const std::vector<Tensor>& hamiltonian);
    
    // Truncation
    void truncate(size_t max_bond_dim, double tolerance);
    
    // Accessors
    const std::vector<Tensor>& get_tensors() const { return tensors_; }
    size_t get_bond_dim() const { return bond_dim_; }
    
private:
    std::vector<Tensor> tensors_;
    size_t bond_dim_;
    size_t num_sites_;
};

/// Projected Entangled Pair State (PEPS)
class PEPS {
public:
    PEPS(size_t width, size_t height, size_t bond_dim);
    
    // Evolution methods
    void evolve(double dt, const std::vector<Tensor>& hamiltonian);
    
    // Truncation
    void truncate(size_t max_bond_dim, double tolerance);
    
private:
    std::vector<std::vector<Tensor>> tensors_;
    size_t bond_dim_;
    size_t width_;
    size_t height_;
};

/// Tensor Evolution Engine
class TensorEvolutionEngine {
public:
    TensorEvolutionEngine();
    
    // Evolution algorithms
    void evolve_tebd(MPS& mps, double dt, size_t num_steps, 
                     const std::vector<Tensor>& hamiltonian);
    void evolve_tdvp(MPS& mps, double dt, size_t num_steps,
                     const std::vector<Tensor>& hamiltonian);
    
    // Multi-GPU support
    void set_gpu_devices(const std::vector<int>& gpu_ids);
    
private:
    std::vector<int> gpu_devices_;
};

} // namespace apeiron::solver::tensor

