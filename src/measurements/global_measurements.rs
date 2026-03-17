// Global measurements
//
// Collection of global ECG measurements.
// Original author: Maarten JB van Ettinger.

use crate::measurements::{GlobalMeasurement, Spike};

/// QTc calculation type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QTcCalcType {
    #[default]
    Unknown,
    Bazett,
    Hodges,
    Fridericia,
    Framingham,
}

/// Collection of global ECG measurements.
#[derive(Clone, Debug)]
pub struct GlobalMeasurements {
    qtc: u16,
    vent_rate: u16,
    pub avg_rr: u16,
    pub avg_pp: u16,
    pub measurement: Option<Vec<GlobalMeasurement>>,
    pub spike: Option<Vec<Spike>>,
}

impl Default for GlobalMeasurements {
    fn default() -> Self {
        Self {
            qtc: GlobalMeasurement::NO_VALUE,
            vent_rate: GlobalMeasurement::NO_VALUE,
            avg_rr: GlobalMeasurement::NO_VALUE,
            avg_pp: GlobalMeasurement::NO_VALUE,
            measurement: None,
            spike: None,
        }
    }
}

impl GlobalMeasurements {
    /// Get ventricular rate.
    pub fn vent_rate(&self) -> u16 {
        if self.vent_rate < GlobalMeasurement::NO_VALUE {
            return self.vent_rate;
        }
        if self.avg_rr == 0 || self.avg_rr == GlobalMeasurement::NO_VALUE {
            0
        } else {
            60000 / self.avg_rr
        }
    }

    /// Set ventricular rate.
    pub fn set_vent_rate(&mut self, val: u16) {
        self.vent_rate = if val < GlobalMeasurement::NO_VALUE {
            val
        } else {
            GlobalMeasurement::NO_VALUE
        };
    }

    /// Get P duration (from first measurement).
    pub fn p_dur(&self) -> u16 {
        self.first_measurement().map_or(GlobalMeasurement::NO_VALUE, |m| m.p_dur())
    }

    /// Set P duration.
    pub fn set_p_dur(&mut self, value: u16) {
        self.ensure_first_measurement();
        if let Some(ref mut ms) = self.measurement {
            ms[0].set_p_dur(value);
        }
    }

    /// Get PR interval.
    pub fn pr_int(&self) -> u16 {
        self.first_measurement().map_or(GlobalMeasurement::NO_VALUE, |m| m.pr_int())
    }

    /// Set PR interval.
    pub fn set_pr_int(&mut self, value: u16) {
        self.ensure_first_measurement();
        if let Some(ref mut ms) = self.measurement {
            ms[0].set_pr_int(value);
        }
    }

    /// Get QRS duration.
    pub fn qrs_dur(&self) -> u16 {
        self.first_measurement().map_or(GlobalMeasurement::NO_VALUE, |m| m.qrs_dur())
    }

    /// Set QRS duration.
    pub fn set_qrs_dur(&mut self, value: u16) {
        self.ensure_first_measurement();
        if let Some(ref mut ms) = self.measurement {
            ms[0].set_qrs_dur(value);
        }
    }

    /// Get QT duration.
    pub fn qt_dur(&self) -> u16 {
        self.first_measurement().map_or(GlobalMeasurement::NO_VALUE, |m| m.qt_dur())
    }

    /// Set QT duration.
    pub fn set_qt_dur(&mut self, value: u16) {
        self.ensure_first_measurement();
        if let Some(ref mut ms) = self.measurement {
            ms[0].set_qt_dur(value);
        }
    }

    /// Get corrected QT interval.
    pub fn qtc(&self) -> u16 {
        if self.qtc < GlobalMeasurement::NO_VALUE {
            return self.qtc;
        }
        if let Some(ref ms) = self.measurement {
            if !ms.is_empty() && self.avg_rr != GlobalMeasurement::NO_VALUE {
                return ms[0].calc_qtc(self.avg_rr, self.vent_rate(), self.qtc_type());
            }
        }
        GlobalMeasurement::NO_VALUE
    }

    /// Set corrected QT interval.
    pub fn set_qtc(&mut self, value: u16) {
        self.qtc = value;
    }

    /// Get QTc calculation type.
    pub fn qtc_type(&self) -> QTcCalcType {
        if self.qtc >= GlobalMeasurement::NO_VALUE {
            let diff = self.qtc - GlobalMeasurement::NO_VALUE;
            match diff {
                0 => QTcCalcType::Bazett,
                1 => QTcCalcType::Hodges,
                2 => QTcCalcType::Fridericia,
                3 => QTcCalcType::Framingham,
                _ => QTcCalcType::Unknown,
            }
        } else {
            QTcCalcType::Unknown
        }
    }

    /// Set QTc calculation type.
    pub fn set_qtc_type(&mut self, calc_type: QTcCalcType) {
        if calc_type != QTcCalcType::Unknown {
            self.qtc = GlobalMeasurement::NO_VALUE + match calc_type {
                QTcCalcType::Bazett => 0,
                QTcCalcType::Hodges => 1,
                QTcCalcType::Fridericia => 2,
                QTcCalcType::Framingham => 3,
                QTcCalcType::Unknown => 0,
            };
        } else if self.qtc >= GlobalMeasurement::NO_VALUE {
            self.qtc = 0;
        }
    }

    fn first_measurement(&self) -> Option<&GlobalMeasurement> {
        self.measurement.as_ref().and_then(|ms| ms.first())
    }

    fn ensure_first_measurement(&mut self) {
        if self.measurement.is_none() {
            self.measurement = Some(vec![GlobalMeasurement::default()]);
        }
    }
}

/// Interface for manipulation of global measurements.
pub trait GlobalMeasurementProvider {
    fn get_global_measurements(&self) -> Option<&GlobalMeasurements>;
    fn set_global_measurements(&mut self, mes: GlobalMeasurements) -> i32;
}
