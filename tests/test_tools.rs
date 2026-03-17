// Tests for the tools module: ecg_tool functions.

use ecgtoolkit::tools::ecg_tool;

// ── lead_subtract / lead_add ────────────────────────────────────────────

#[test]
fn lead_subtract_basic() {
    let mut a = vec![100, 200, 300, 400];
    let b = vec![10, 20, 30, 40];
    assert!(ecg_tool::lead_subtract(&mut a, &b));
    assert_eq!(a, vec![90, 180, 270, 360]);
}

#[test]
fn lead_subtract_negative() {
    let mut a = vec![10, 20];
    let b = vec![20, 30];
    assert!(ecg_tool::lead_subtract(&mut a, &b));
    assert_eq!(a, vec![-10, -10]);
}

#[test]
fn lead_subtract_different_lengths() {
    let mut a = vec![1, 2, 3];
    let b = vec![1, 2];
    assert!(!ecg_tool::lead_subtract(&mut a, &b));
}

#[test]
fn lead_add_basic() {
    let mut a = vec![100, 200, 300];
    let b = vec![10, 20, 30];
    assert!(ecg_tool::lead_add(&mut a, &b));
    assert_eq!(a, vec![110, 220, 330]);
}

#[test]
fn lead_add_different_lengths() {
    let mut a = vec![1, 2];
    let b = vec![1, 2, 3];
    assert!(!ecg_tool::lead_add(&mut a, &b));
}

// ── Lead calculations ───────────────────────────────────────────────────

#[test]
fn calculate_lead_iii() {
    // III = II - I
    let lead_i = vec![100, 200, 300, 400, 500];
    let lead_ii = vec![150, 250, 350, 450, 550];
    let result = ecg_tool::calculate_lead_iii(&lead_i, 0, 5, &lead_ii, 0, 5, 5).unwrap();
    assert_eq!(result, vec![50, 50, 50, 50, 50]);
}

#[test]
fn calculate_lead_iii_with_offset() {
    let lead_i = vec![100, 200];
    let lead_ii = vec![300, 400];
    let result = ecg_tool::calculate_lead_iii(&lead_i, 1, 2, &lead_ii, 1, 2, 4).unwrap();
    // Position 0: both out of range → 0
    // Position 1: lead_i[0]=100, lead_ii[0]=300 → 200
    // Position 2: lead_i[1]=200, lead_ii[1]=400 → 200
    // Position 3: both out of range → 0
    assert_eq!(result, vec![0, 200, 200, 0]);
}

#[test]
fn calculate_lead_avr() {
    // aVR = -(I + II) / 2
    let lead_i = vec![100, 200, 300];
    let lead_ii = vec![200, 300, 400];
    let result = ecg_tool::calculate_lead_avr(&lead_i, 0, 3, &lead_ii, 0, 3, 3).unwrap();
    // -(100+200)/2 = -150, -(200+300)/2 = -250, -(300+400)/2 = -350
    assert_eq!(result, vec![-150, -250, -350]);
}

#[test]
fn calculate_lead_avl_two_lead() {
    // aVL (two-lead) = (2*I - II) / 2
    let lead_i = vec![200, 400];
    let lead_ii = vec![100, 200];
    let result = ecg_tool::calculate_lead_avl(&lead_i, 0, 2, &lead_ii, 0, 2, 2, false).unwrap();
    // (2*200-100)/2 = 150, (2*400-200)/2 = 300
    assert_eq!(result, vec![150, 300]);
}

#[test]
fn calculate_lead_avl_three_lead() {
    // aVL (three-lead) = (I - III) / 2
    let lead_i = vec![200, 400];
    let lead_iii = vec![100, 200];
    let result = ecg_tool::calculate_lead_avl(&lead_i, 0, 2, &lead_iii, 0, 2, 2, true).unwrap();
    // (200-100)/2 = 50, (400-200)/2 = 100
    assert_eq!(result, vec![50, 100]);
}

#[test]
fn calculate_lead_avf_two_lead() {
    // aVF (two-lead) = (2*II - I) / 2
    let lead_i = vec![100, 200];
    let lead_ii = vec![300, 400];
    let result = ecg_tool::calculate_lead_avf(&lead_i, 0, 2, &lead_ii, 0, 2, 2, false).unwrap();
    // (2*300-100)/2 = 250, (2*400-200)/2 = 300
    assert_eq!(result, vec![250, 300]);
}

#[test]
fn calculate_lead_avf_three_lead() {
    // aVF (three-lead) = (II + III) / 2
    let lead_iii = vec![100, 200];
    let lead_ii = vec![300, 400];
    let result = ecg_tool::calculate_lead_avf(&lead_iii, 0, 2, &lead_ii, 0, 2, 2, true).unwrap();
    // (300+100)/2 = 200, (400+200)/2 = 300
    assert_eq!(result, vec![200, 300]);
}

#[test]
fn calculate_leads_from_two() {
    let lead_i = vec![100; 10];
    let lead_ii = vec![200; 10];
    let result = ecg_tool::calculate_leads_from_two(&lead_i, &lead_ii, 10).unwrap();
    assert_eq!(result.len(), 4); // III, aVR, aVL, aVF

    // III = II - I = 100
    assert_eq!(result[0][0], 100);
    // aVR = -(I+II)/2 = -150
    assert_eq!(result[1][0], -150);
    // aVL = (2*I-II)/2 = 0
    assert_eq!(result[2][0], 0);
    // aVF = (2*II-I)/2 = 150
    assert_eq!(result[3][0], 150);
}

#[test]
fn calculate_leads_from_two_empty() {
    assert!(ecg_tool::calculate_leads_from_two(&[], &[1], 1).is_none());
    assert!(ecg_tool::calculate_leads_from_two(&[1], &[], 1).is_none());
    assert!(ecg_tool::calculate_leads_from_two(&[1], &[1], 0).is_none());
}

#[test]
fn calculate_leads_from_three() {
    let lead_i = vec![100; 5];
    let lead_ii = vec![200; 5];
    let lead_iii = vec![100; 5]; // III = II - I = 100
    let result = ecg_tool::calculate_leads_from_three(&lead_i, &lead_ii, &lead_iii, 5).unwrap();
    assert_eq!(result.len(), 3); // aVR, aVL, aVF
}

#[test]
fn calculate_leads_from_three_empty() {
    assert!(ecg_tool::calculate_leads_from_three(&[], &[1], &[1], 1).is_none());
}

// ── Einthoven's law verification ────────────────────────────────────────

#[test]
fn einthoven_law_i_plus_iii_equals_ii() {
    // Einthoven's law: I + III = II
    let lead_i = vec![100, -50, 200, 0, -100];
    let lead_ii = vec![250, 100, 350, 150, -50];
    let result = ecg_tool::calculate_leads_from_two(&lead_i, &lead_ii, 5).unwrap();
    let lead_iii = &result[0];

    for i in 0..5 {
        assert_eq!(lead_i[i] + lead_iii[i], lead_ii[i],
            "Einthoven's law failed at sample {}: {} + {} != {}",
            i, lead_i[i], lead_iii[i], lead_ii[i]);
    }
}

// ── Resampling ──────────────────────────────────────────────────────────

#[test]
fn resample_same_frequency() {
    let src = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = ecg_tool::resample_lead(&src, 500, 500).unwrap();
    assert_eq!(result, src);
}

#[test]
fn resample_upsample_2x() {
    // Constant signal should remain constant after resampling
    let src = vec![100; 100];
    let result = ecg_tool::resample_lead(&src, 250, 500).unwrap();
    // Upsampled by 2x, should have ~200 samples
    assert!(result.len() >= 199 && result.len() <= 201,
        "Expected ~200, got {}", result.len());
    // Interior samples (skip boundaries where interpolation edge effects occur)
    // should be close to 100
    let interior = &result[5..result.len().saturating_sub(5)];
    for &val in interior {
        assert!((val - 100).abs() <= 5, "Expected ~100, got {}", val);
    }
}

#[test]
fn resample_downsample_2x() {
    let src = vec![100; 200];
    let result = ecg_tool::resample_lead(&src, 500, 250).unwrap();
    assert!(result.len() >= 100 && result.len() <= 102,
        "Expected ~100, got {}", result.len());
    // Interior samples should be close to 100
    let interior = &result[3..result.len().saturating_sub(3)];
    for &val in interior {
        assert!((val - 100).abs() <= 5, "Expected ~100, got {}", val);
    }
}

#[test]
fn resample_invalid_inputs() {
    assert!(ecg_tool::resample_lead(&[], 500, 500).is_none());
    assert!(ecg_tool::resample_lead(&[1], 0, 500).is_none());
    assert!(ecg_tool::resample_lead(&[1], 500, 0).is_none());
    assert!(ecg_tool::resample_lead(&[1], -1, 500).is_none());
}

#[test]
fn resample_range_subset() {
    let src = vec![0, 0, 100, 100, 100, 0, 0];
    let result = ecg_tool::resample_lead_range(&src, 2, 3, 500, 500).unwrap();
    assert_eq!(result, vec![100, 100, 100]);
}

#[test]
fn resample_range_out_of_bounds() {
    let src = vec![1, 2, 3];
    assert!(ecg_tool::resample_lead_range(&src, 2, 5, 500, 500).is_none());
}

// ── change_multiplier ───────────────────────────────────────────────────

#[test]
fn change_multiplier_double() {
    let mut data = vec![100, 200, -100, -200];
    let ret = ecg_tool::change_multiplier(&mut data, 2.5, 5.0);
    assert_eq!(ret, 0);
    // (val * 2.5) / 5.0 = val / 2
    assert_eq!(data, vec![50, 100, -50, -100]);
}

#[test]
fn change_multiplier_same() {
    let mut data = vec![100, 200];
    let ret = ecg_tool::change_multiplier(&mut data, 2.5, 2.5);
    assert_eq!(ret, 0);
    assert_eq!(data, vec![100, 200]); // unchanged
}

#[test]
fn change_multiplier_invalid() {
    let mut data = vec![100];
    assert_eq!(ecg_tool::change_multiplier(&mut data, 0.0, 5.0), 1);
    assert_eq!(ecg_tool::change_multiplier(&mut data, 5.0, 0.0), 1);
    assert_eq!(ecg_tool::change_multiplier(&mut data, -1.0, 5.0), 1);
}

// ── copy_signal ─────────────────────────────────────────────────────────

#[test]
fn copy_signal_basic() {
    let src = vec![10, 20, 30, 40, 50];
    let mut dst = vec![0; 5];
    let ret = ecg_tool::copy_signal(&src, 1, &mut dst, 2, 3);
    assert_eq!(ret, 0);
    assert_eq!(dst, vec![0, 0, 20, 30, 40]);
}

#[test]
fn copy_signal_truncate_src() {
    let src = vec![10, 20, 30];
    let mut dst = vec![0; 10];
    // Request len=5 but src only has 3-1=2 samples from offset 1
    let ret = ecg_tool::copy_signal(&src, 1, &mut dst, 0, 5);
    assert_eq!(ret, 0);
    assert_eq!(dst[0], 20);
    assert_eq!(dst[1], 30);
}

#[test]
fn copy_signal_dst_too_small() {
    let src = vec![10, 20, 30];
    let mut dst = vec![0; 2];
    let ret = ecg_tool::copy_signal(&src, 0, &mut dst, 0, 3);
    assert_eq!(ret, 1);
}

// ── shift_signal ────────────────────────────────────────────────────────

#[test]
fn shift_signal_positive() {
    let mut data = vec![100, 200, 300];
    ecg_tool::shift_signal(&mut data, 50);
    assert_eq!(data, vec![50, 150, 250]);
}

#[test]
fn shift_signal_negative() {
    let mut data = vec![100, 200, 300];
    ecg_tool::shift_signal(&mut data, -50);
    assert_eq!(data, vec![150, 250, 350]);
}

#[test]
fn shift_signal_zero() {
    let mut data = vec![100, 200, 300];
    ecg_tool::shift_signal(&mut data, 0);
    assert_eq!(data, vec![100, 200, 300]); // unchanged
}
