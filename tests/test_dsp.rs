// Tests for the DSP module: filters.

use ecgtoolkit::dsp::*;

// ── FIR filter ──────────────────────────────────────────────────────────

#[test]
fn fir_filter_passthrough() {
    let mut fir = FirFilter::new(3);
    // Identity: a = [1, 0, 0]
    let a = [1.0, 0.0, 0.0];
    assert_eq!(fir.compute(5.0, &a), 5.0);
    assert_eq!(fir.compute(10.0, &a), 10.0);
    assert_eq!(fir.compute(-3.0, &a), -3.0);
}

#[test]
fn fir_filter_averaging() {
    let mut fir = FirFilter::new(3);
    // Simple averaging: a = [0.5, 0.5, 0]
    let a = [0.5, 0.5, 0.0];
    let r1 = fir.compute(10.0, &a);
    assert_eq!(r1, 5.0); // 0.5*10 + 0.5*0 = 5
    let r2 = fir.compute(20.0, &a);
    assert_eq!(r2, 15.0); // 0.5*20 + 0.5*10 = 15
}

#[test]
fn fir_filter_delay_line() {
    let mut fir = FirFilter::new(3);
    // Pure delay: a = [0, 1, 0]
    let a = [0.0, 1.0, 0.0];
    assert_eq!(fir.compute(100.0, &a), 0.0);  // delayed by 1
    assert_eq!(fir.compute(200.0, &a), 100.0); // previous input
    assert_eq!(fir.compute(300.0, &a), 200.0);
}

// ── IIR filter ──────────────────────────────────────────────────────────

#[test]
fn iir_filter_no_feedback() {
    let mut iir = IirFilter::new(2);
    let a = [0.0, 0.0]; // no feedback
    assert_eq!(iir.compute(5.0, &a), 5.0);
    assert_eq!(iir.compute(10.0, &a), 10.0);
}

#[test]
fn iir_filter_with_feedback() {
    let mut iir = IirFilter::new(2);
    let a = [0.5, 0.0]; // y(t) = x(t) + 0.5*y(t-1)
    let r1 = iir.compute(10.0, &a);
    assert_eq!(r1, 10.0); // 10 + 0.5*0 = 10
    let r2 = iir.compute(0.0, &a);
    assert_eq!(r2, 5.0); // 0 + 0.5*10 = 5
    let r3 = iir.compute(0.0, &a);
    assert_eq!(r3, 2.5); // 0 + 0.5*5 = 2.5
}

// ── Lowpass Butterworth ─────────────────────────────────────────────────

#[test]
fn lowpass_filter_dc_passthrough() {
    // DC signal (all same value) should pass through a lowpass filter
    let mut filter = LowpassFilterButterworth::new(50.0, 2, 500.0);
    // Warm up
    for _ in 0..100 {
        filter.compute(1000.0);
    }
    let output = filter.compute(1000.0);
    // After settling, DC should pass through approximately unchanged
    assert!((output - 1000.0).abs() < 1.0, "DC output was {}", output);
}

#[test]
fn lowpass_filter_attenuates_high_frequency() {
    let fs = 500.0;
    let cutoff = 10.0;
    let mut filter = LowpassFilterButterworth::new(cutoff, 2, fs);

    // Generate high frequency signal (200 Hz, well above cutoff)
    let freq = 200.0;
    let mut max_output = 0.0f64;
    for i in 0..1000 {
        let t = i as f64 / fs;
        let input = 1000.0 * (2.0 * std::f64::consts::PI * freq * t).sin();
        let output = filter.compute(input);
        if i > 200 { // skip transient
            max_output = max_output.max(output.abs());
        }
    }
    // High frequency should be significantly attenuated
    assert!(max_output < 100.0, "High freq max was {}", max_output);
}

// ── Highpass Butterworth ────────────────────────────────────────────────

#[test]
fn highpass_filter_blocks_dc() {
    let mut filter = HighpassFilterButterworth::new(1.0, 2, 500.0);
    // Feed DC for a while
    for _ in 0..1000 {
        filter.compute(1000.0);
    }
    let output = filter.compute(1000.0);
    // DC should be blocked (close to 0, allow small residual)
    assert!(output.abs() < 5.0, "DC should be blocked, got {}", output);
}

#[test]
fn highpass_filter_passes_high_frequency() {
    let fs = 500.0;
    let cutoff = 1.0;
    let mut filter = HighpassFilterButterworth::new(cutoff, 2, fs);

    let freq = 50.0; // well above cutoff
    let mut max_output = 0.0f64;
    for i in 0..2000 {
        let t = i as f64 / fs;
        let input = 1000.0 * (2.0 * std::f64::consts::PI * freq * t).sin();
        let output = filter.compute(input);
        if i > 500 { // skip transient
            max_output = max_output.max(output.abs());
        }
    }
    // High frequency should pass through mostly unchanged
    assert!(max_output > 900.0, "High freq should pass, max was {}", max_output);
}

// ── Bandpass Butterworth ────────────────────────────────────────────────

#[test]
fn bandpass_filter_passes_midband() {
    let fs = 500.0;
    let mut filter = BandpassFilterButterworth::new(5.0, 50.0, 2, fs);

    let freq = 20.0; // in the passband
    let mut max_output = 0.0f64;
    for i in 0..2000 {
        let t = i as f64 / fs;
        let input = 1000.0 * (2.0 * std::f64::consts::PI * freq * t).sin();
        let output = filter.compute(input);
        if i > 500 {
            max_output = max_output.max(output.abs());
        }
    }
    assert!(max_output > 800.0, "Midband should pass, max was {}", max_output);
}

#[test]
fn bandpass_filter_blocks_dc() {
    let mut filter = BandpassFilterButterworth::new(5.0, 50.0, 2, 500.0);
    for _ in 0..1000 {
        filter.compute(1000.0);
    }
    let output = filter.compute(1000.0);
    assert!(output.abs() < 1.0, "DC should be blocked, got {}", output);
}

#[test]
fn bandpass_filter_attenuates_high_frequency() {
    let fs = 500.0;
    let mut filter = BandpassFilterButterworth::new(5.0, 50.0, 2, fs);

    let freq = 200.0; // above passband
    let mut max_output = 0.0f64;
    for i in 0..1000 {
        let t = i as f64 / fs;
        let input = 1000.0 * (2.0 * std::f64::consts::PI * freq * t).sin();
        let output = filter.compute(input);
        if i > 200 {
            max_output = max_output.max(output.abs());
        }
    }
    assert!(max_output < 100.0, "High freq should be attenuated, max was {}", max_output);
}

// ── Butterworth section internals ───────────────────────────────────────

#[test]
fn lowpass_section_compute() {
    let mut section = LowpassFilterButterworthSection::new(50.0, 1.0, 4.0, 500.0);
    // Just verify it doesn't panic and produces finite output
    let output = section.compute(100.0);
    assert!(output.is_finite());
}

#[test]
fn highpass_section_compute() {
    let mut section = HighpassFilterButterworthSection::new(1.0, 1.0, 4.0, 500.0);
    let output = section.compute(100.0);
    assert!(output.is_finite());
}

// ── Filter trait ────────────────────────────────────────────────────────

#[test]
fn filter_trait_object() {
    let mut filter: Box<dyn Filter> = Box::new(LowpassFilterButterworth::new(50.0, 2, 500.0));
    let output = filter.compute(100.0);
    assert!(output.is_finite());
}

#[test]
fn filter_clone() {
    let f1 = LowpassFilterButterworth::new(50.0, 2, 500.0);
    let mut f2 = f1.clone();
    let output = f2.compute(100.0);
    assert!(output.is_finite());
}
