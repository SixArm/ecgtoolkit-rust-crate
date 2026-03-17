// Bandpass filter Butterworth implementation
//
// Cascade of lowpass and highpass Butterworth filters.
// Original author: Maarten JB van Ettinger.

use crate::dsp::{Filter, HighpassFilterButterworth, LowpassFilterButterworth};

/// Bandpass filter as cascade of lowpass and highpass Butterworth filters.
#[derive(Clone, Debug)]
pub struct BandpassFilterButterworth {
    lowpass_filter: LowpassFilterButterworth,
    highpass_filter: HighpassFilterButterworth,
}

impl BandpassFilterButterworth {
    pub fn new(
        bottom_frequency_hz: f64,
        top_frequency_hz: f64,
        num_sections: usize,
        fs: f64,
    ) -> Self {
        Self {
            lowpass_filter: LowpassFilterButterworth::new(top_frequency_hz, num_sections, fs),
            highpass_filter: HighpassFilterButterworth::new(bottom_frequency_hz, num_sections, fs),
        }
    }
}

impl Filter for BandpassFilterButterworth {
    fn compute(&mut self, input: f64) -> f64 {
        let lp_output = self.lowpass_filter.compute(input);
        self.highpass_filter.compute(lp_output)
    }
}
