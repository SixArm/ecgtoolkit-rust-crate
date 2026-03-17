// Signals
//
// Container for all ECG signal leads.
// Original author: Maarten JB van Ettinger.

use crate::dsp::{
    BandpassFilterButterworth, HighpassFilterButterworth, LowpassFilterButterworth,
};
use crate::signal::{QrsZone, Signal};
use crate::tools::ecg_tool;
use crate::types::LeadType;

/// Container for all ECG signal leads.
#[derive(Clone, Debug, Default)]
pub struct Signals {
    /// Rhythm analog-to-digital multiplier in mV.
    pub rhythm_avm: f64,
    /// Rhythm samples per second.
    pub rhythm_samples_per_second: i32,

    /// Median analog-to-digital multiplier in mV.
    pub median_avm: f64,
    /// Median length in ms.
    pub median_length: u16,
    /// Median samples per second.
    pub median_samples_per_second: i32,

    /// Median fiducial point.
    pub median_fiducial_point: u16,
    /// QRS zones.
    pub qrs_zone: Option<Vec<QrsZone>>,

    /// Signal leads.
    leads: Option<Vec<Signal>>,
}

impl Signals {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nr_leads(nr_leads: u8) -> Self {
        let mut s = Self::default();
        s.set_nr_leads(nr_leads);
        s
    }

    pub fn nr_leads(&self) -> u8 {
        self.leads.as_ref().map_or(0, |l| l.len() as u8)
    }

    pub fn set_nr_leads(&mut self, nr: u8) {
        let mut leads = Vec::with_capacity(nr as usize);
        leads.resize_with(nr as usize, Signal::default);
        self.leads = Some(leads);
    }

    pub fn get_leads(&self) -> Option<&[Signal]> {
        self.leads.as_deref()
    }

    pub fn get_leads_mut(&mut self) -> Option<&mut Vec<Signal>> {
        self.leads.as_mut()
    }

    pub fn set_leads(&mut self, leads: Vec<Signal>) {
        if leads.len() <= u8::MAX as usize {
            self.leads = Some(leads);
        }
    }

    pub fn get(&self, i: usize) -> Option<&Signal> {
        self.leads.as_ref().and_then(|l| l.get(i))
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Signal> {
        self.leads.as_mut().and_then(|l| l.get_mut(i))
    }

    /// Determine if the first eight leads are as expected (I, II, V1-V6).
    pub fn is_normal(&self) -> bool {
        self.leads.as_ref().map_or(false, |l| Signal::is_normal(l))
    }

    /// Calculate start and end of signals.
    pub fn calculate_start_and_end(&self) -> (i32, i32) {
        let mut n_start = i32::MAX;
        let mut n_end = i32::MIN;

        if let Some(ref leads) = self.leads {
            for lead in leads {
                if lead.rhythm_start < n_start {
                    n_start = lead.rhythm_start;
                }
                if lead.rhythm_end > n_end {
                    n_end = lead.rhythm_end;
                }
            }
        }

        (n_start, n_end)
    }

    /// Number of simultaneously recorded leads.
    pub fn nr_simultaneously(&self) -> usize {
        self.leads.as_ref().map_or(0, |l| Signal::nr_simultaneously(l))
    }

    /// Sort leads by lead type.
    pub fn sort_on_type(&mut self) {
        if let Some(ref mut leads) = self.leads {
            Signal::sort_on_type(leads);
        }
    }

    /// Trim signals by removing leading/trailing samples matching `val`.
    pub fn trim_signals(&mut self, val: i16) {
        let (start, end) = self.calculate_start_and_end();
        self.trim_signals_range(val, start, end);
    }

    /// Trim signals with explicit start/end.
    pub fn trim_signals_range(&mut self, val: i16, start: i32, end: i32) {
        let sps = self.rhythm_samples_per_second;
        if let Some(ref mut leads) = self.leads {
            for sig in leads.iter_mut() {
                if let Some(ref mut rhythm) = sig.rhythm {
                    let len = rhythm.len();
                    let mut trim_begin = 0usize;
                    let mut trim_end = len.saturating_sub(1);

                    if sig.rhythm_start == start {
                        for i in 0..len {
                            if rhythm[i] != val {
                                trim_begin = i;
                                break;
                            }
                        }
                    }

                    if sig.rhythm_end == end {
                        for i in (1..len).rev() {
                            if rhythm[i] != val {
                                trim_end = i;
                                break;
                            }
                        }
                    }

                    if sps > 0 && (trim_begin as i32 / sps) < 1 {
                        trim_begin = 0;
                    }

                    if sps > 0 && ((len.saturating_sub(1) - trim_end) as i32 / sps) < 1 {
                        trim_end = len.saturating_sub(1);
                    }

                    if trim_begin != 0 || trim_end != len.saturating_sub(1) {
                        sig.rhythm_start += trim_begin as i32;
                        sig.rhythm_end -= (len.saturating_sub(1) - trim_end) as i32;
                        *rhythm = rhythm[trim_begin..=trim_end].to_vec();
                    }
                }
            }
        }
    }

    /// Apply bandpass filter to produce a filtered copy.
    pub fn apply_bandpass_filter(&self, bottom: f64, top: f64) -> Option<Signals> {
        self.apply_bandpass_filter_sections(bottom, top, 2)
    }

    /// Apply bandpass filter with specified number of sections.
    pub fn apply_bandpass_filter_sections(
        &self,
        bottom: f64,
        top: f64,
        num_sections: usize,
    ) -> Option<Signals> {
        let mut sigs = self.clone_metadata();

        if let Some(ref leads) = self.leads {
            sigs.set_nr_leads(self.nr_leads());
            let sigs_leads = sigs.get_leads_mut()?;

            for (i, lead) in leads.iter().enumerate() {
                let mut rhythm_filter: Option<Box<dyn crate::dsp::Filter>> = None;
                let mut median_filter: Option<Box<dyn crate::dsp::Filter>> = None;

                if lead.rhythm.is_some() && self.rhythm_samples_per_second > 0 {
                    rhythm_filter = Some(Box::new(BandpassFilterButterworth::new(
                        bottom, top, num_sections, self.rhythm_samples_per_second as f64,
                    )));
                }

                if lead.median.is_some() && self.median_samples_per_second > 0 {
                    median_filter = Some(Box::new(BandpassFilterButterworth::new(
                        bottom, top, num_sections, self.median_samples_per_second as f64,
                    )));
                }

                let filtered = lead.apply_filter(rhythm_filter, median_filter)?;
                sigs_leads[i] = filtered;
            }
        }

        Some(sigs)
    }

    /// Apply lowpass filter to produce a filtered copy.
    pub fn apply_lowpass_filter(&self, cutoff: f64) -> Option<Signals> {
        self.apply_lowpass_filter_sections(cutoff, 2)
    }

    /// Apply lowpass filter with specified number of sections.
    pub fn apply_lowpass_filter_sections(
        &self,
        cutoff: f64,
        num_sections: usize,
    ) -> Option<Signals> {
        let mut sigs = self.clone_metadata();

        if let Some(ref leads) = self.leads {
            sigs.set_nr_leads(self.nr_leads());
            let sigs_leads = sigs.get_leads_mut()?;

            for (i, lead) in leads.iter().enumerate() {
                let mut rhythm_filter: Option<Box<dyn crate::dsp::Filter>> = None;
                let mut median_filter: Option<Box<dyn crate::dsp::Filter>> = None;

                if lead.rhythm.is_some() && self.rhythm_samples_per_second > 0 {
                    rhythm_filter = Some(Box::new(LowpassFilterButterworth::new(
                        cutoff, num_sections, self.rhythm_samples_per_second as f64,
                    )));
                }

                if lead.median.is_some() && self.median_samples_per_second > 0 {
                    median_filter = Some(Box::new(LowpassFilterButterworth::new(
                        cutoff, num_sections, self.median_samples_per_second as f64,
                    )));
                }

                let filtered = lead.apply_filter(rhythm_filter, median_filter)?;
                sigs_leads[i] = filtered;
            }
        }

        Some(sigs)
    }

    /// Apply highpass filter to produce a filtered copy.
    pub fn apply_highpass_filter(&self, cutoff: f64) -> Option<Signals> {
        self.apply_highpass_filter_sections(cutoff, 2)
    }

    /// Apply highpass filter with specified number of sections.
    pub fn apply_highpass_filter_sections(
        &self,
        cutoff: f64,
        num_sections: usize,
    ) -> Option<Signals> {
        let mut sigs = self.clone_metadata();

        if let Some(ref leads) = self.leads {
            sigs.set_nr_leads(self.nr_leads());
            let sigs_leads = sigs.get_leads_mut()?;

            for (i, lead) in leads.iter().enumerate() {
                let mut rhythm_filter: Option<Box<dyn crate::dsp::Filter>> = None;
                let mut median_filter: Option<Box<dyn crate::dsp::Filter>> = None;

                if lead.rhythm.is_some() && self.rhythm_samples_per_second > 0 {
                    rhythm_filter = Some(Box::new(HighpassFilterButterworth::new(
                        cutoff, num_sections, self.rhythm_samples_per_second as f64,
                    )));
                }

                if lead.median.is_some() && self.median_samples_per_second > 0 {
                    median_filter = Some(Box::new(HighpassFilterButterworth::new(
                        cutoff, num_sections, self.median_samples_per_second as f64,
                    )));
                }

                let filtered = lead.apply_filter(rhythm_filter, median_filter)?;
                sigs_leads[i] = filtered;
            }
        }

        Some(sigs)
    }

    /// Resample all leads to a new sample rate.
    pub fn resample(&mut self, samples_per_second: i32) {
        if let Some(ref mut leads) = self.leads {
            for sig in leads.iter_mut() {
                if self.rhythm_samples_per_second != 0
                    && self.rhythm_avm != 0.0
                    && sig.rhythm.is_some()
                {
                    if let Some(ref rhythm) = sig.rhythm {
                        if let Some(resampled) = ecg_tool::resample_lead(
                            rhythm,
                            self.rhythm_samples_per_second,
                            samples_per_second,
                        ) {
                            sig.rhythm = Some(resampled);
                            sig.rhythm_start = ((sig.rhythm_start as i64
                                * samples_per_second as i64)
                                / self.rhythm_samples_per_second as i64)
                                as i32;
                            sig.rhythm_end = ((sig.rhythm_end as i64
                                * samples_per_second as i64)
                                / self.rhythm_samples_per_second as i64)
                                as i32;
                        }
                    }
                }

                if self.median_samples_per_second != 0
                    && self.median_avm != 0.0
                    && sig.median.is_some()
                {
                    if let Some(ref median) = sig.median {
                        if let Some(resampled) = ecg_tool::resample_lead(
                            median,
                            self.median_samples_per_second,
                            samples_per_second,
                        ) {
                            sig.median = Some(resampled);
                        }
                    }
                }
            }
        }

        if let Some(ref mut qrs_zones) = self.qrs_zone {
            for zone in qrs_zones.iter_mut() {
                zone.start = ((zone.start as i64 * samples_per_second as i64)
                    / self.median_samples_per_second as i64) as i32;
                zone.fiducial = ((zone.fiducial as i64 * samples_per_second as i64)
                    / self.median_samples_per_second as i64) as i32;
                zone.end = ((zone.end as i64 * samples_per_second as i64)
                    / self.median_samples_per_second as i64) as i32;
            }
        }

        if self.rhythm_samples_per_second != 0 && self.rhythm_avm != 0.0 {
            self.rhythm_samples_per_second = samples_per_second;
        }

        if self.median_samples_per_second != 0 && self.median_avm != 0.0 {
            self.median_fiducial_point = ((self.median_fiducial_point as i64
                * samples_per_second as i64)
                / self.median_samples_per_second as i64) as u16;
            self.median_samples_per_second = samples_per_second;
        }
    }

    /// Set AVM (analog-to-digital multiplier) for all signals.
    pub fn set_avm(&mut self, avm: f64) {
        if avm == 0.0 {
            return;
        }

        let nr_leads = self.nr_leads() as usize;

        for i in 0..nr_leads {
            if let Some(ref mut leads) = self.leads {
                if let Some(ref mut rhythm) = leads[i].rhythm {
                    ecg_tool::change_multiplier(rhythm, self.rhythm_avm, avm);
                }
                if let Some(ref mut median) = leads[i].median {
                    ecg_tool::change_multiplier(median, self.median_avm, avm);
                }
            }
        }

        if self.rhythm_avm != 0.0 {
            self.rhythm_avm = avm;
        }
        if self.median_avm != 0.0 {
            self.median_avm = avm;
        }
    }

    /// Determine whether this is a twelve-lead signal.
    pub fn is_twelve_leads(&self) -> bool {
        let twelve_lead_types = [
            LeadType::I, LeadType::II, LeadType::III,
            LeadType::AVR, LeadType::AVL, LeadType::AVF,
            LeadType::V1, LeadType::V2, LeadType::V3,
            LeadType::V4, LeadType::V5, LeadType::V6,
        ];

        let nr_sim = self.nr_simultaneously();
        let leads = match self.leads.as_ref() {
            Some(l) => l,
            None => return false,
        };

        if nr_sim != leads.len() {
            return false;
        }

        if nr_sim == twelve_lead_types.len() {
            for i in 0..nr_sim {
                if leads[i].lead_type != twelve_lead_types[i] {
                    return false;
                }
            }
            return true;
        } else if nr_sim == 15 {
            for i in 0..twelve_lead_types.len() {
                if leads[i].lead_type != twelve_lead_types[i] {
                    return false;
                }
            }
            let extra: &[&[LeadType]] = &[
                &[LeadType::V3R, LeadType::V4R, LeadType::V7],
                &[LeadType::V7, LeadType::V8, LeadType::V9],
            ];
            for pattern in extra {
                let mut matches = true;
                for (j, i) in (twelve_lead_types.len()..nr_sim).enumerate() {
                    if leads[i].lead_type != pattern[j] {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    return true;
                }
            }
        }

        false
    }

    /// Determine whether this is a fifteen-lead signal.
    pub fn is_fifteen_leads(&self) -> bool {
        let lt1 = [
            LeadType::I, LeadType::II, LeadType::III,
            LeadType::AVR, LeadType::AVL, LeadType::AVF,
            LeadType::V1, LeadType::V2, LeadType::V3,
            LeadType::V4, LeadType::V5, LeadType::V6,
            LeadType::V3R, LeadType::V4R, LeadType::V7,
        ];
        let lt2 = [
            LeadType::I, LeadType::II, LeadType::III,
            LeadType::AVR, LeadType::AVL, LeadType::AVF,
            LeadType::V1, LeadType::V2, LeadType::V3,
            LeadType::V4, LeadType::V5, LeadType::V6,
            LeadType::V7, LeadType::V8, LeadType::V9,
        ];

        let nr_sim = self.nr_simultaneously();
        let leads = match self.leads.as_ref() {
            Some(l) => l,
            None => return false,
        };

        if nr_sim != leads.len() || nr_sim != 15 {
            return false;
        }

        for pattern in &[&lt1[..], &lt2[..]] {
            let mut matches = true;
            for i in 0..nr_sim {
                if leads[i].lead_type != pattern[i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                return true;
            }
        }

        false
    }

    /// Calculate twelve leads from available leads.
    pub fn calculate_twelve_leads(&self) -> Option<Signals> {
        let twelve_lead_types = [
            LeadType::I, LeadType::II, LeadType::III,
            LeadType::AVR, LeadType::AVL, LeadType::AVF,
            LeadType::V1, LeadType::V2, LeadType::V3,
            LeadType::V4, LeadType::V5, LeadType::V6,
        ];

        let nr_sim = self.nr_simultaneously();
        let leads = self.leads.as_ref()?;

        if nr_sim != leads.len() {
            return None;
        }

        if nr_sim == 8 {
            // Calculate from 8 leads (I, II, V1-V6)
            let eight_lead_types = [
                LeadType::I, LeadType::II,
                LeadType::V1, LeadType::V2, LeadType::V3,
                LeadType::V4, LeadType::V5, LeadType::V6,
            ];

            let mut pos: Vec<Option<Signal>> = vec![None; 12];

            // Map existing leads to positions
            let mut found = vec![false; 8];
            for lead in leads.iter() {
                for (j, &lt) in eight_lead_types.iter().enumerate() {
                    if lead.lead_type == lt && !found[j] {
                        found[j] = true;
                        let target_idx = twelve_lead_types.iter()
                            .position(|&t| t == lead.lead_type)?;
                        pos[target_idx] = Some(lead.clone());
                        break;
                    }
                }
            }

            if found.iter().any(|&f| !f) {
                return None;
            }

            // Calculate derived leads (III, aVR, aVL, aVF)
            let lead_i = pos[0].as_ref()?.rhythm.as_ref()?;
            let lead_ii = pos[1].as_ref()?.rhythm.as_ref()?;
            let total_length = lead_i.len();

            let calc_leads = ecg_tool::calculate_leads_from_two(
                lead_i, lead_ii, total_length,
            )?;

            let rhythm_start = pos[0].as_ref()?.rhythm_start;
            let rhythm_end = pos[0].as_ref()?.rhythm_end;

            for (i, calc_lead) in calc_leads.into_iter().enumerate() {
                let mut sig = Signal::default();
                sig.lead_type = twelve_lead_types[i + 2]; // III, aVR, aVL, aVF
                sig.rhythm_start = rhythm_start;
                sig.rhythm_end = rhythm_end;
                sig.rhythm = Some(calc_lead);
                pos[i + 2] = Some(sig);
            }

            // Also calculate median if available
            let median_i = pos[0].as_ref()?.median.as_ref();
            let median_ii = pos[1].as_ref()?.median.as_ref();
            if let (Some(mi), Some(mii)) = (median_i, median_ii) {
                if let Some(calc_medians) = ecg_tool::calculate_leads_from_two(
                    mi, mii, mi.len(),
                ) {
                    for (i, calc_median) in calc_medians.into_iter().enumerate() {
                        if let Some(ref mut sig) = pos[i + 2] {
                            sig.median = Some(calc_median);
                        }
                    }
                }
            }

            let final_leads: Vec<Signal> = pos.into_iter()
                .map(|s| s.unwrap_or_default())
                .collect();

            let mut sigs = self.clone();
            sigs.set_leads(final_leads);
            return Some(sigs);
        }

        None
    }

    /// Clone only metadata (not leads).
    fn clone_metadata(&self) -> Signals {
        Signals {
            rhythm_avm: self.rhythm_avm,
            rhythm_samples_per_second: self.rhythm_samples_per_second,
            median_avm: self.median_avm,
            median_length: self.median_length,
            median_samples_per_second: self.median_samples_per_second,
            median_fiducial_point: self.median_fiducial_point,
            qrs_zone: self.qrs_zone.clone(),
            leads: None,
        }
    }
}
