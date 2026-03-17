// FIR filter implementation
//
// Finite Impulse Response filter.
// Original author: Maarten JB van Ettinger.

/// Finite Impulse Response (FIR) filter.
#[derive(Clone, Debug)]
pub struct FirFilter {
    z: Vec<f64>,
}

impl FirFilter {
    pub fn new(order: usize) -> Self {
        Self {
            z: vec![0.0; order],
        }
    }

    /// Compute y(t) = a0*x(t) + a1*x(t-1) + a2*x(t-2) + ... an*x(t-n)
    pub fn compute(&mut self, input: f64, a: &[f64]) -> f64 {
        let mut result = 0.0;

        for t in (0..a.len()).rev() {
            if t > 0 {
                self.z[t] = self.z[t - 1];
            } else {
                self.z[t] = input;
            }
            result += a[t] * self.z[t];
        }
        result
    }
}
