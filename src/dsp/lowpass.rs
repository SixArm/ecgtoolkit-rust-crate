// Lowpass filter Butterworth implementation
//
// Multi-section Butterworth lowpass filter.
// Original author: Maarten JB van Ettinger.

use crate::dsp::{Filter, LowpassFilterButterworthSection};

/// Multi-section Butterworth lowpass filter.
#[derive(Clone, Debug)]
pub struct LowpassFilterButterworth {
    sections: Vec<LowpassFilterButterworthSection>,
}

impl LowpassFilterButterworth {
    pub fn new(cutoff_frequency_hz: f64, num_sections: usize, fs: f64) -> Self {
        let sections = (0..num_sections)
            .map(|i| {
                LowpassFilterButterworthSection::new(
                    cutoff_frequency_hz,
                    (i + 1) as f64,
                    (num_sections * 2) as f64,
                    fs,
                )
            })
            .collect();

        Self { sections }
    }
}

impl Filter for LowpassFilterButterworth {
    fn compute(&mut self, input: f64) -> f64 {
        let mut output = input;
        for section in &mut self.sections {
            output = section.compute(output);
        }
        output
    }
}
