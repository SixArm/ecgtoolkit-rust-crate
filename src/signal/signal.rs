// Signal
//
// Data of one ECG signal (lead/channel).
// Original author: Maarten JB van Ettinger.

use crate::dsp::Filter;
use crate::types::LeadType;

/// Data of one ECG signal (lead/channel).
#[derive(Clone, Debug, Default)]
pub struct Signal {
    pub lead_type: LeadType,
    pub rhythm_start: i32,
    pub rhythm_end: i32,
    pub rhythm: Option<Vec<i16>>,
    pub median: Option<Vec<i16>>,
}

impl Signal {
    /// Apply filters to produce a filtered copy of this signal.
    pub fn apply_filter(
        &self,
        mut rhythm_filter: Option<Box<dyn Filter>>,
        mut median_filter: Option<Box<dyn Filter>>,
    ) -> Option<Signal> {
        let mut sig = Signal {
            lead_type: self.lead_type,
            rhythm_start: self.rhythm_start,
            rhythm_end: self.rhythm_end,
            rhythm: None,
            median: None,
        };

        if let Some(ref rhythm) = self.rhythm {
            let filter = rhythm_filter.as_mut()?;
            // Prime the filter
            filter.compute(rhythm[0] as f64);
            filter.compute(rhythm[0] as f64);
            sig.rhythm = Some(
                rhythm.iter()
                    .map(|&s| filter.compute(s as f64).round() as i16)
                    .collect()
            );
        }

        if let Some(ref median) = self.median {
            let filter = median_filter.as_mut()?;
            // Prime the filter
            filter.compute(median[0] as f64);
            filter.compute(median[0] as f64);
            sig.median = Some(
                median.iter()
                    .map(|&s| filter.compute(s as f64).round() as i16)
                    .collect()
            );
        }

        Some(sig)
    }

    /// Determine if the first eight leads are as expected (I, II, V1-V6).
    pub fn is_normal(data: &[Signal]) -> bool {
        if data.len() >= 8 {
            for i in 0..8 {
                if data[i].lead_type != LeadType::from_u8((1 + i) as u8) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Determine the number of simultaneously recorded leads.
    pub fn nr_simultaneously(data: &[Signal]) -> usize {
        if data.len() <= 1 {
            return 0;
        }

        let allowed_diff = 5;
        let mut nr = 1;

        while nr < data.len() {
            if (data[0].rhythm_start - data[nr].rhythm_start).unsigned_abs() > allowed_diff
                || (data[0].rhythm_end - data[nr].rhythm_end).unsigned_abs() > allowed_diff
            {
                break;
            }
            nr += 1;
        }
        nr
    }

    /// Sort signal slice on lead type (quicksort).
    pub fn sort_on_type(data: &mut [Signal]) {
        if data.len() > 1 {
            Self::sort_on_type_range(data, 0, data.len() - 1);
        }
    }

    fn sort_on_type_range(data: &mut [Signal], first: usize, last: usize) {
        if first < last {
            let p = Self::partition_on_type(data, first, last);
            if p > 0 {
                Self::sort_on_type_range(data, first, p - 1);
            }
            Self::sort_on_type_range(data, p + 1, last);
        }
    }

    fn partition_on_type(data: &mut [Signal], first: usize, last: usize) -> usize {
        let m = (first + last) / 2;
        data.swap(m, first);

        let pivot_type = data[first].lead_type;
        let mut p = first;

        for i in (first + 1)..=last {
            if data[i].lead_type < pivot_type {
                p += 1;
                data.swap(p, i);
            }
        }

        data.swap(first, p);
        p
    }
}
