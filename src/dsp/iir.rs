// IIR filter implementation
//
// Infinite Impulse Response filter.
// Original author: Maarten JB van Ettinger.

/// Infinite Impulse Response (IIR) filter.
#[derive(Clone, Debug)]
pub struct IirFilter {
    z: Vec<f64>,
}

impl IirFilter {
    pub fn new(order: usize) -> Self {
        Self {
            z: vec![0.0; order],
        }
    }

    /// Compute y(t) = x(t) + a1*y(t-1) + a2*y(t-2) + ... an*y(t-n)
    ///
    /// z-transform: H(z) = 1 / (1 - sum(1 to n) [an * y(t-n)])
    /// a0 is assumed to be 1.
    pub fn compute(&mut self, input: f64, a: &[f64]) -> f64 {
        let mut result = input;

        for t in 0..a.len() {
            result += a[t] * self.z[t];
        }

        for t in (0..a.len()).rev() {
            if t > 0 {
                self.z[t] = self.z[t - 1];
            } else {
                self.z[t] = result;
            }
        }
        result
    }
}
