// Global measurement
//
// One wave measurement (SCP and UNIPRO defined).
// Original author: Maarten JB van Ettinger.

use crate::measurements::global_measurements::QTcCalcType;

/// One wave measurement (SCP and UNIPRO defined).
#[derive(Clone, Debug)]
pub struct GlobalMeasurement {
    pub p_onset: u16,
    pub p_offset: u16,
    pub qrs_onset: u16,
    pub qrs_offset: u16,
    pub t_offset: u16,
    pub p_axis: i16,
    pub qrs_axis: i16,
    pub t_axis: i16,
}

impl GlobalMeasurement {
    pub const NO_VALUE: u16 = 29999;
    pub const NO_AXIS_VALUE: i16 = 29999;

    /// P-wave duration.
    pub fn p_dur(&self) -> u16 {
        if self.p_offset != Self::NO_VALUE
            && self.p_onset != Self::NO_VALUE
            && self.p_onset < self.p_offset
        {
            self.p_offset - self.p_onset
        } else {
            Self::NO_VALUE
        }
    }

    /// Set P-wave duration.
    pub fn set_p_dur(&mut self, value: u16) {
        if value > 0 && value != Self::NO_VALUE {
            if self.p_onset == Self::NO_VALUE {
                self.p_onset = 100;
            }
            self.p_offset = value + self.p_onset;
        } else {
            self.p_onset = Self::NO_VALUE;
            self.p_offset = Self::NO_VALUE;
        }
    }

    /// PR interval.
    pub fn pr_int(&self) -> u16 {
        if self.qrs_onset != Self::NO_VALUE && self.p_onset != Self::NO_VALUE {
            self.qrs_onset - self.p_onset
        } else {
            Self::NO_VALUE
        }
    }

    /// Set PR interval.
    pub fn set_pr_int(&mut self, value: u16) {
        if value > 0 && value != Self::NO_VALUE {
            if self.p_onset == Self::NO_VALUE {
                self.p_onset = 100;
                self.p_offset = Self::NO_VALUE;
            }
            self.qrs_onset = value + self.p_onset;
        }
    }

    /// QRS duration.
    pub fn qrs_dur(&self) -> u16 {
        if self.qrs_offset != Self::NO_VALUE && self.qrs_onset != Self::NO_VALUE {
            self.qrs_offset - self.qrs_onset
        } else {
            Self::NO_VALUE
        }
    }

    /// Set QRS duration.
    pub fn set_qrs_dur(&mut self, value: u16) {
        if value != Self::NO_VALUE && value != 0 {
            if self.qrs_onset == Self::NO_VALUE || self.qrs_onset == 0 {
                self.p_onset = Self::NO_VALUE;
                self.p_offset = Self::NO_VALUE;
                self.qrs_onset = 400;
            }
            self.qrs_offset = value + self.qrs_onset;
        }
    }

    /// QT duration.
    pub fn qt_dur(&self) -> u16 {
        if self.t_offset != Self::NO_VALUE && self.qrs_onset != Self::NO_VALUE {
            self.t_offset - self.qrs_onset
        } else {
            Self::NO_VALUE
        }
    }

    /// Set QT duration.
    pub fn set_qt_dur(&mut self, value: u16) {
        if value != Self::NO_VALUE
            && value != 0
            && self.qrs_onset != Self::NO_VALUE
            && self.qrs_onset != 0
        {
            self.t_offset = self.qrs_onset + value;
        } else {
            self.t_offset = Self::NO_VALUE;
        }
    }

    /// T-wave duration.
    pub fn t_dur(&self) -> u16 {
        if self.t_offset != Self::NO_VALUE && self.p_onset != Self::NO_VALUE {
            self.t_offset - self.qrs_offset
        } else {
            Self::NO_VALUE
        }
    }

    /// Calculate corrected QT interval.
    pub fn calc_qtc(&self, avg_rr: u16, hr: u16, calc_type: QTcCalcType) -> u16 {
        if avg_rr == 0 || avg_rr == Self::NO_VALUE || self.qt_dur() == Self::NO_VALUE {
            return Self::NO_VALUE;
        }

        let qt = self.qt_dur() as f64;
        let rr_sec = avg_rr as f64 * 0.001;

        match calc_type {
            QTcCalcType::Bazett => (qt / rr_sec.sqrt()) as u16,
            QTcCalcType::Fridericia => (qt / rr_sec.powf(1.0 / 3.0)) as u16,
            QTcCalcType::Framingham => (qt + 154.0 * (1.0 - rr_sec)) as u16,
            QTcCalcType::Hodges => (qt + 1.75 * (hr as f64 - 60.0)) as u16,
            QTcCalcType::Unknown => Self::NO_VALUE,
        }
    }
}

impl Default for GlobalMeasurement {
    fn default() -> Self {
        Self {
            p_onset: Self::NO_VALUE,
            p_offset: Self::NO_VALUE,
            qrs_onset: Self::NO_VALUE,
            qrs_offset: Self::NO_VALUE,
            t_offset: Self::NO_VALUE,
            p_axis: Self::NO_AXIS_VALUE,
            qrs_axis: Self::NO_AXIS_VALUE,
            t_axis: Self::NO_AXIS_VALUE,
        }
    }
}
