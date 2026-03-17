// Lead measurements
//
// Container for per-lead ECG measurements.
// Original author: Maarten JB van Ettinger.

use crate::measurements::LeadMeasurement;

/// Container for per-lead ECG measurements.
#[derive(Clone, Debug, Default)]
pub struct LeadMeasurements {
    pub measurements: Option<Vec<LeadMeasurement>>,
}

impl LeadMeasurements {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_count(nr: usize) -> Self {
        let measurements: Vec<LeadMeasurement> = (0..nr)
            .map(|_| LeadMeasurement::new())
            .collect();
        Self {
            measurements: Some(measurements),
        }
    }
}

/// Interface for manipulation of lead measurements.
pub trait LeadMeasurementProvider {
    fn get_lead_measurements(&self) -> Option<&LeadMeasurements>;
    fn set_lead_measurements(&mut self, mes: LeadMeasurements) -> i32;
}
