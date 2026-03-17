// Spike
//
// Spike data (SCP defined).
// Original author: Maarten JB van Ettinger.

use crate::measurements::global_measurement::GlobalMeasurement;

/// One spike (SCP defined).
#[derive(Clone, Debug)]
pub struct Spike {
    pub time: u16,
    pub amplitude: i16,
}

impl Default for Spike {
    fn default() -> Self {
        Self {
            time: GlobalMeasurement::NO_VALUE,
            amplitude: GlobalMeasurement::NO_AXIS_VALUE,
        }
    }
}
