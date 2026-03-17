// ECG tool
//
// Utility functions for lead calculations, resampling, and signal processing.
// Original author: Maarten JB van Ettinger.

use crate::demographics::Demographic;

/// Constant limit for the holter resample algorithm.
pub const MAX_HOLTER_SPS: i32 = 25;
pub const MIN_HOLTER_SPS: i32 = 10;

/// Interval to use for resampling with polynomial in msec.
pub const RESAMPLE_INTERVAL: i32 = 20;

/// Subtract lead B from lead A (in place).
pub fn lead_subtract(lead_a: &mut [i16], lead_b: &[i16]) -> bool {
    if lead_a.len() == lead_b.len() {
        for i in 0..lead_a.len() {
            lead_a[i] = lead_a[i].wrapping_sub(lead_b[i]);
        }
        true
    } else {
        false
    }
}

/// Add lead B to lead A (in place).
pub fn lead_add(lead_a: &mut [i16], lead_b: &[i16]) -> bool {
    if lead_a.len() == lead_b.len() {
        for i in 0..lead_a.len() {
            lead_a[i] = lead_a[i].wrapping_add(lead_b[i]);
        }
        true
    } else {
        false
    }
}

/// Calculate four leads (III, aVR, aVL, aVF) from lead I and II.
///
/// Returns vec of [III, aVR, aVL, aVF] or None on error.
pub fn calculate_leads_from_two(
    lead_i: &[i16],
    lead_ii: &[i16],
    total_length: usize,
) -> Option<Vec<Vec<i16>>> {
    if lead_i.is_empty() || lead_ii.is_empty() || total_length == 0 {
        return None;
    }

    let lead_iii = calculate_lead_iii(lead_i, 0, lead_i.len(), lead_ii, 0, lead_ii.len(), total_length)?;
    let lead_avr = calculate_lead_avr(lead_i, 0, lead_i.len(), lead_ii, 0, lead_ii.len(), total_length)?;
    let lead_avl = calculate_lead_avl(lead_i, 0, lead_i.len(), lead_ii, 0, lead_ii.len(), total_length, false)?;
    let lead_avf = calculate_lead_avf(lead_i, 0, lead_i.len(), lead_ii, 0, lead_ii.len(), total_length, false)?;

    Some(vec![lead_iii, lead_avr, lead_avl, lead_avf])
}

/// Calculate four leads (aVR, aVL, aVF) from lead I, II, and III.
///
/// Returns vec of [aVR, aVL, aVF] or None on error.
pub fn calculate_leads_from_three(
    lead_i: &[i16],
    lead_ii: &[i16],
    lead_iii: &[i16],
    total_length: usize,
) -> Option<Vec<Vec<i16>>> {
    if lead_i.is_empty() || lead_ii.is_empty() || lead_iii.is_empty() || total_length == 0 {
        return None;
    }

    let avr = calculate_lead_avr(lead_i, 0, lead_i.len(), lead_ii, 0, lead_ii.len(), total_length)?;
    let avl = calculate_lead_avl(lead_i, 0, lead_i.len(), lead_iii, 0, lead_iii.len(), total_length, true)?;
    let avf = calculate_lead_avf(lead_iii, 0, lead_iii.len(), lead_ii, 0, lead_ii.len(), total_length, true)?;

    Some(vec![avr, avl, avf])
}

/// Calculate lead III from lead I and II.
/// III = II - I
pub fn calculate_lead_iii(
    lead_i: &[i16], begin_i: usize, length_i: usize,
    lead_ii: &[i16], begin_ii: usize, length_ii: usize,
    total_length: usize,
) -> Option<Vec<i16>> {
    let mut ret = vec![0i16; total_length];

    for i in 0..total_length {
        let data_i = if i >= begin_i && i < begin_i + length_i {
            lead_i[i - begin_i]
        } else {
            0
        };
        let data_ii = if i >= begin_ii && i < begin_ii + length_ii {
            lead_ii[i - begin_ii]
        } else {
            0
        };
        ret[i] = (data_ii as i32 - data_i as i32) as i16;
    }

    Some(ret)
}

/// Calculate lead aVR from lead I and II.
/// aVR = -(I + II) / 2
pub fn calculate_lead_avr(
    lead_i: &[i16], begin_i: usize, length_i: usize,
    lead_ii: &[i16], begin_ii: usize, length_ii: usize,
    total_length: usize,
) -> Option<Vec<i16>> {
    let mut ret = vec![0i16; total_length];

    for i in 0..total_length {
        let data_i = if i >= begin_i && i < begin_i + length_i {
            lead_i[i - begin_i] as i32
        } else {
            0
        };
        let data_ii = if i >= begin_ii && i < begin_ii + length_ii {
            lead_ii[i - begin_ii] as i32
        } else {
            0
        };
        ret[i] = (-((data_i + data_ii) >> 1)) as i16;
    }

    Some(ret)
}

/// Calculate lead aVL from lead I and II/III.
/// When three_lead: aVL = (I - III) / 2
/// When two_lead: aVL = (2*I - II) / 2
pub fn calculate_lead_avl(
    lead_i: &[i16], begin_i: usize, length_i: usize,
    lead_x: &[i16], begin_x: usize, length_x: usize,
    total_length: usize,
    three_lead: bool,
) -> Option<Vec<i16>> {
    let mut ret = vec![0i16; total_length];

    for i in 0..total_length {
        let data_i = if i >= begin_i && i < begin_i + length_i {
            lead_i[i - begin_i] as i32
        } else {
            0
        };
        let data_x = if i >= begin_x && i < begin_x + length_x {
            lead_x[i - begin_x] as i32
        } else {
            0
        };

        ret[i] = if three_lead {
            ((data_i - data_x) >> 1) as i16
        } else {
            (((data_i << 1) - data_x) >> 1) as i16
        };
    }

    Some(ret)
}

/// Calculate lead aVF from lead I/III and II.
/// When three_lead: aVF = (II + III) / 2
/// When two_lead: aVF = (2*II - I) / 2
pub fn calculate_lead_avf(
    lead_x: &[i16], begin_x: usize, length_x: usize,
    lead_ii: &[i16], begin_ii: usize, length_ii: usize,
    total_length: usize,
    three_lead: bool,
) -> Option<Vec<i16>> {
    let mut ret = vec![0i16; total_length];

    for i in 0..total_length {
        let data_x = if i >= begin_x && i < begin_x + length_x {
            lead_x[i - begin_x] as i32
        } else {
            0
        };
        let data_ii = if i >= begin_ii && i < begin_ii + length_ii {
            lead_ii[i - begin_ii] as i32
        } else {
            0
        };

        ret[i] = if three_lead {
            ((data_ii + data_x) >> 1) as i16
        } else {
            (((data_ii << 1) - data_x) >> 1) as i16
        };
    }

    Some(ret)
}

/// Resample a single lead.
pub fn resample_lead(
    src: &[i16],
    src_freq: i32,
    dst_freq: i32,
) -> Option<Vec<i16>> {
    resample_lead_range(src, 0, src.len(), src_freq, dst_freq)
}

/// Resample a single lead with offset and length.
pub fn resample_lead_range(
    src: &[i16],
    startsample: usize,
    nrsamples: usize,
    src_freq: i32,
    dst_freq: i32,
) -> Option<Vec<i16>> {
    if src.is_empty() || src_freq <= 0 || dst_freq <= 0 || nrsamples == 0 {
        return None;
    }

    if startsample + nrsamples > src.len() {
        return None;
    }

    if src_freq == dst_freq {
        return Some(src[startsample..startsample + nrsamples].to_vec());
    }

    let dst_size = (nrsamples as i64 * dst_freq as i64 / src_freq as i64 + 1) as usize;
    let mut dst = vec![0i16; dst_size];

    let zwischen_freq = lcm(src_freq, dst_freq);
    let src_add = zwischen_freq / src_freq;
    let dst_add = zwischen_freq / dst_freq;

    let start = (startsample as i64 * src_add as i64) as i64;
    let end = ((startsample + nrsamples) as i64 * src_add as i64) as i64;

    let n = {
        let mut n = (RESAMPLE_INTERVAL * src_freq) / 1000;
        n >>= 1;
        if n <= 0 { n = 1; }
        n <<= 1;
        n
    };

    let mut zwischen = start;
    while zwischen < end {
        let dst_idx = ((zwischen - start) / dst_add as i64) as usize;
        if dst_idx >= dst.len() {
            break;
        }

        if (zwischen % src_add as i64) == 0 {
            let src_idx = (zwischen / src_add as i64) as usize;
            if src_idx < src.len() {
                dst[dst_idx] = src[src_idx];
            }
        } else {
            // Polynomial interpolation
            let center = (zwischen / src_add as i64) as i32;
            let first = center - (n >> 1);
            let mut used_n = n;

            let mut actual_first = first;
            if actual_first < -1 {
                used_n -= (-1 - actual_first) << 1;
                actual_first = -1;
            }
            if actual_first + used_n >= nrsamples as i32 {
                used_n -= ((actual_first + used_n) - nrsamples as i32) << 1;
                actual_first = nrsamples as i32 - used_n - 1;
            }

            if used_n <= 0 {
                zwischen += dst_add as i64;
                continue;
            }

            // Neville's algorithm
            let mut c = vec![0.0f64; (used_n + 1) as usize];
            let mut d = vec![0.0f64; (used_n + 1) as usize];

            let mut ns = 1i32;
            let mut dif = (zwischen - ((actual_first + 1) as i64 * src_add as i64)).unsigned_abs();

            for l in 1..=used_n {
                let dift = (zwischen - ((actual_first + l) as i64 * src_add as i64)).unsigned_abs();
                if dift < dif {
                    ns = l;
                    dif = dift;
                }
                let idx = (actual_first + l) as usize;
                let val = if idx < src.len() { src[idx] as f64 } else { 0.0 };
                c[l as usize] = val;
                d[l as usize] = val;
            }

            let idx = (actual_first + ns) as usize;
            let mut y = if idx < src.len() { src[idx] as f64 } else { 0.0 };
            ns -= 1;

            for l1 in 1..used_n {
                for l2 in 1..=(used_n - l1) {
                    let ho = ((actual_first + l2) as i64 * src_add as i64) - zwischen;
                    let hp = ((actual_first + l2 + l1) as i64 * src_add as i64) - zwischen;
                    let w = c[(l2 + 1) as usize] - d[l2 as usize];
                    let den = ho - hp;
                    if den == 0 {
                        zwischen += dst_add as i64;
                        continue;
                    }
                    let den = w / den as f64;
                    d[l2 as usize] = hp as f64 * den;
                    c[l2 as usize] = ho as f64 * den;
                }

                if (ns << 1) < (used_n - l1) {
                    y += c[(ns + 1) as usize];
                } else {
                    y += d[ns as usize];
                    ns -= 1;
                }
            }

            dst[dst_idx] = y as i16;
        }

        zwischen += dst_add as i64;
    }

    Some(dst)
}

/// Change multiplier for a signal (in place).
pub fn change_multiplier(src: &mut [i16], src_multi: f64, dst_multi: f64) -> i32 {
    if src_multi == dst_multi {
        return 0;
    }
    if src_multi <= 0.0 || dst_multi <= 0.0 {
        return 1;
    }

    for sample in src.iter_mut() {
        *sample = ((*sample as f64 * src_multi) / dst_multi) as i16;
    }
    0
}

/// Copy signal data.
pub fn copy_signal(
    src: &[i16], src_offset: usize,
    dst: &mut [i16], dst_offset: usize,
    mut len: usize,
) -> i32 {
    if src_offset + len > src.len() {
        len = src.len().saturating_sub(src_offset);
    }
    if len == 0 || dst_offset + len > dst.len() {
        return 1;
    }

    dst[dst_offset..dst_offset + len].copy_from_slice(&src[src_offset..src_offset + len]);
    0
}

/// Shift signal by subtracting a value from each sample.
pub fn shift_signal(src: &mut [i16], shift: i16) -> i32 {
    if shift != 0 {
        for sample in src.iter_mut() {
            *sample -= shift;
        }
    }
    0
}

/// Anonymize demographics.
pub fn anonymous(demo: &mut dyn Demographic, ch: char) {
    if let Some(name) = demo.last_name() {
        let anon = ch.to_string().repeat(name.len());
        demo.set_last_name(Some(&anon));
    }

    if let Some(name) = demo.first_name() {
        let anon = ch.to_string().repeat(name.len());
        demo.set_first_name(Some(&anon));
    }

    if let Some(id) = demo.patient_id() {
        let anon = ch.to_string().repeat(id.len());
        demo.set_patient_id(Some(&anon));
    }

    if let Some(name) = demo.second_last_name() {
        let anon = ch.to_string().repeat(name.len());
        demo.set_second_last_name(Some(&anon));
    }

    if let Some(name) = demo.prefix_name() {
        let anon = ch.to_string().repeat(name.len());
        demo.set_prefix_name(Some(&anon));
    }

    if let Some(name) = demo.suffix_name() {
        let anon = ch.to_string().repeat(name.len());
        demo.set_suffix_name(Some(&anon));
    }

    if let Some(date) = demo.patient_birth_date() {
        let mut new_date = date.clone();
        new_date.day = 1;
        new_date.month = 1;
        demo.set_patient_birth_date(Some(new_date));
    }

    if demo.sequence_nr().is_some() {
        demo.set_sequence_nr(Some("1"));
    }
}

/// Greatest common divisor.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Least common multiple.
fn lcm(a: i32, b: i32) -> i32 {
    let g = gcd(a, b);
    if g == 0 { 0 } else { (a * b) / g }
}
