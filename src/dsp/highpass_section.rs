// Highpass filter Butterworth section
//
// One section of a Butterworth highpass filter.
// Original author: Maarten JB van Ettinger.

use std::f64::consts::PI;

use crate::dsp::{FirFilter, IirFilter};

/// One section of a Butterworth highpass filter.
#[derive(Clone, Debug)]
pub struct HighpassFilterButterworthSection {
    fir_filter: FirFilter,
    iir_filter: IirFilter,
    a: [f64; 3],
    b: [f64; 2],
    gain: f64,
}

impl HighpassFilterButterworthSection {
    pub fn new(cutoff_frequency_hz: f64, k: f64, n: f64, fs: f64) -> Self {
        // Pre-warp omegac and invert it
        let omegac = 1.0 / (2.0 * fs * (PI * cutoff_frequency_hz / fs).tan());
        let zeta = -(PI * (2.0 * k + n - 1.0) / (2.0 * n)).cos();

        let a = [
            4.0 * fs * fs,
            -8.0 * fs * fs,
            4.0 * fs * fs,
        ];

        let b0 = (4.0 * fs * fs) + (4.0 * fs * zeta / omegac) + (1.0 / (omegac * omegac));
        let b = [
            ((2.0 / (omegac * omegac)) - (8.0 * fs * fs)) / (-b0),
            ((4.0 * fs * fs) - (4.0 * fs * zeta / omegac) + (1.0 / (omegac * omegac))) / (-b0),
        ];
        let gain = 1.0 / b0;

        Self {
            fir_filter: FirFilter::new(3),
            iir_filter: IirFilter::new(2),
            a,
            b,
            gain,
        }
    }

    pub fn compute(&mut self, input: f64) -> f64 {
        let fir_output = self.fir_filter.compute(self.gain * input, &self.a);
        self.iir_filter.compute(fir_output, &self.b)
    }
}
