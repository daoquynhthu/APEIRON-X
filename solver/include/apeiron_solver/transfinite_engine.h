#pragma once

#include <vector>
#include <cstdint>

namespace apeiron::solver::transfinite {

/// Ordinal level
using Ordinal = uint64_t;

/// Transfinite Recursor
class TransfiniteRecursor {
public:
    TransfiniteRecursor();
    
    /// Process ordinal alpha layer expansion
    std::vector<double> expand_ordinal_layer(Ordinal alpha,
                                               const std::vector<double>& initial_state);
    
    /// Reset evolution time tau = 0
    void reset_evolution_time();
    
    /// Detect fixed point
    bool detect_fixed_point(const std::vector<double>& state,
                             const std::vector<double>& previous_state,
                             double tolerance);
    
    /// Get current evolution time
    double get_evolution_time() const { return tau_; }
    
private:
    double tau_;
    Ordinal current_ordinal_;
};

} // namespace apeiron::solver::transfinite

