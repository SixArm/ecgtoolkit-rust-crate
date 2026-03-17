// Lowpass filter Butterworth section
//
// One section of a Butterworth lowpass filter.
// Original author: Maarten JB van Ettinger.

use std::f64::consts::PI;

use crate::dsp::{FirFilter, IirFilter};

/// One section of a Butterworth lowpass filter.
#[derive(Clone, Debug)]
pub struct LowpassFilterButterworthSection {
    fir_filter: FirFilter,
    iir_filter: IirFilter,
    a: [f64; 3],
    b: [f64; 2],
    gain: f64,
}

impl LowpassFilterButterworthSection {
    pub fn new(cutoff_frequency_hz: f64, k: f64, n: f64, fs: f64) -> Self {
        let omegac = 2.0 * fs * (PI * cutoff_frequency_hz / fs).tan();
        let zeta = -(PI * (2.0 * k + n - 1.0) / (2.0 * n)).cos();

        let a = [
            omegac * omegac,
            2.0 * omegac * omegac,
            omegac * omegac,
        ];

        let b0 = (4.0 * fs * fs) + (4.0 * fs * zeta * omegac) + (omegac * omegac);
        let b = [
            ((2.0 * omegac * omegac) - (8.0 * fs * fs)) / (-b0),
            ((4.0 * fs * fs) - (4.0 * fs * zeta * omegac) + (omegac * omegac)) / (-b0),
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
