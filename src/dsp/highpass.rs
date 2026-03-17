// Highpass filter Butterworth implementation
//
// Multi-section Butterworth highpass filter.
// Original author: Maarten JB van Ettinger.

use crate::dsp::{Filter, HighpassFilterButterworthSection};

/// Multi-section Butterworth highpass filter.
#[derive(Clone, Debug)]
pub struct HighpassFilterButterworth {
    sections: Vec<HighpassFilterButterworthSection>,
}

impl HighpassFilterButterworth {
    pub fn new(cutoff_frequency_hz: f64, num_sections: usize, fs: f64) -> Self {
        let sections = (0..num_sections)
            .map(|i| {
                HighpassFilterButterworthSection::new(
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

impl Filter for HighpassFilterButterworth {
    fn compute(&mut self, input: f64) -> f64 {
        let mut output = input;
        for section in &mut self.sections {
            output = section.compute(output);
        }
        output
    }
}
