// Lead measurement
//
// Per-lead ECG measurement data.
// Original author: Maarten JB van Ettinger.

use std::collections::BTreeMap;

use crate::measurements::MeasurementType;
use crate::types::LeadType;

/// One lead's measurement data.
#[derive(Clone, Debug, Default)]
pub struct LeadMeasurement {
    pub lead_type: LeadType,
    values: BTreeMap<i32, i16>,
}

impl LeadMeasurement {
    pub const NO_VALUE: i16 = 29999;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_lead_type(lt: LeadType) -> Self {
        Self {
            lead_type: lt,
            values: BTreeMap::new(),
        }
    }

    /// Get measurement value by type.
    pub fn get(&self, mt: MeasurementType) -> i16 {
        self.values.get(&(mt as i32)).copied().unwrap_or(Self::NO_VALUE)
    }

    /// Set measurement value by type.
    pub fn set(&mut self, mt: MeasurementType, value: i16) {
        if value == Self::NO_VALUE {
            self.values.remove(&(mt as i32));
        } else {
            self.values.insert(mt as i32, value);
        }
    }

    /// Number of stored measurements.
    pub fn count(&self) -> usize {
        self.values.len()
    }

    /// Get value by index.
    pub fn get_value_by_index(&self, index: usize) -> i16 {
        self.values.values().nth(index).copied().unwrap_or(Self::NO_VALUE)
    }

    /// Get key by index.
    pub fn get_key_by_index(&self, index: usize) -> MeasurementType {
        self.values.keys().nth(index).map_or(MeasurementType::None, |&k| {
            // Convert i32 back to MeasurementType
            unsafe { std::mem::transmute(k) }
        })
    }
}
