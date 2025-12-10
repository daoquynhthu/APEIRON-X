#pragma once

#include <vector>
#include <complex>
#include <memory>
#include <string>

namespace apeiron::solver::tensor {

using Complex = std::complex<double>;
using Tensor = std::vector<Complex>;

struct SafetyConfig {
    double entropy_threshold = 1e6;     // example upper bound
    double anomaly_threshold = 1e6;     // example upper bound
    size_t max_bond_dim = 4096;         // guard against explosion
    bool require_human_force = false;   // if true, reject dangerous ops
};

struct TensorShape {
    std::vector<size_t> dims;
};

inline bool validate_shape(const TensorShape& s) {
    if (s.dims.empty()) return false;
    for (auto d : s.dims) if (d == 0) return false;
    return true;
}

/// Matrix Product State (MPS)
class MPS {
public:
    MPS(size_t num_sites, size_t bond_dim, const SafetyConfig& safety = {});
    
    // Evolution methods
    bool evolve_tebd(double dt, const std::vector<Tensor>& hamiltonian, const SafetyConfig& safety);
    bool evolve_tdvp(double dt, const std::vector<Tensor>& hamiltonian, const SafetyConfig& safety);
    
    // Truncation
    bool truncate(size_t max_bond_dim, double tolerance, const SafetyConfig& safety);
    
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
    bool evolve_tebd(MPS& mps, double dt, size_t num_steps, 
                     const std::vector<Tensor>& hamiltonian,
                     const SafetyConfig& safety);
    bool evolve_tdvp(MPS& mps, double dt, size_t num_steps,
                     const std::vector<Tensor>& hamiltonian,
                     const SafetyConfig& safety);
    
    // Multi-GPU support
    void set_gpu_devices(const std::vector<int>& gpu_ids);
    
private:
    std::vector<int> gpu_devices_;
    bool validate_hamiltonian(const std::vector<Tensor>& hamiltonian, const SafetyConfig& safety);
};

} // namespace apeiron::solver::tensor

