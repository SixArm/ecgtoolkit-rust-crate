// Filter trait
//
// Interface for digital signal processing filters.
// Original author: Maarten JB van Ettinger.

/// Filter interface for DSP operations.
pub trait Filter {
    fn compute(&mut self, input: f64) -> f64;
}
