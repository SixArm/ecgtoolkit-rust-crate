// Tests for the measurements module.

use ecgtoolkit::measurements::*;
use ecgtoolkit::types::LeadType;

// ── GlobalMeasurement ───────────────────────────────────────────────────

#[test]
fn global_measurement_default() {
    let m = GlobalMeasurement::default();
    assert_eq!(m.p_onset, GlobalMeasurement::NO_VALUE);
    assert_eq!(m.p_offset, GlobalMeasurement::NO_VALUE);
    assert_eq!(m.qrs_onset, GlobalMeasurement::NO_VALUE);
    assert_eq!(m.qrs_offset, GlobalMeasurement::NO_VALUE);
    assert_eq!(m.t_offset, GlobalMeasurement::NO_VALUE);
    assert_eq!(m.p_axis, GlobalMeasurement::NO_AXIS_VALUE);
    assert_eq!(m.qrs_axis, GlobalMeasurement::NO_AXIS_VALUE);
    assert_eq!(m.t_axis, GlobalMeasurement::NO_AXIS_VALUE);
}

#[test]
fn global_measurement_p_dur_no_value_when_default() {
    let m = GlobalMeasurement::default();
    assert_eq!(m.p_dur(), GlobalMeasurement::NO_VALUE);
}

#[test]
fn global_measurement_set_p_dur() {
    let mut m = GlobalMeasurement::default();
    m.set_p_dur(80); // 80ms P-wave
    assert_eq!(m.p_onset, 100); // default onset
    assert_eq!(m.p_offset, 180); // 100 + 80
    assert_eq!(m.p_dur(), 80);
}

#[test]
fn global_measurement_set_p_dur_with_existing_onset() {
    let mut m = GlobalMeasurement::default();
    m.p_onset = 50;
    m.set_p_dur(80);
    assert_eq!(m.p_onset, 50);
    assert_eq!(m.p_offset, 130); // 50 + 80
    assert_eq!(m.p_dur(), 80);
}

#[test]
fn global_measurement_set_p_dur_zero_clears() {
    let mut m = GlobalMeasurement::default();
    m.p_onset = 50;
    m.p_offset = 130;
    m.set_p_dur(0);
    assert_eq!(m.p_onset, GlobalMeasurement::NO_VALUE);
    assert_eq!(m.p_offset, GlobalMeasurement::NO_VALUE);
}

#[test]
fn global_measurement_pr_int() {
    let mut m = GlobalMeasurement::default();
    m.p_onset = 100;
    m.qrs_onset = 260;
    assert_eq!(m.pr_int(), 160);
}

#[test]
fn global_measurement_pr_int_no_value() {
    let m = GlobalMeasurement::default();
    assert_eq!(m.pr_int(), GlobalMeasurement::NO_VALUE);
}

#[test]
fn global_measurement_set_pr_int() {
    let mut m = GlobalMeasurement::default();
    m.set_pr_int(160);
    assert_eq!(m.p_onset, 100); // default
    assert_eq!(m.qrs_onset, 260); // 100 + 160
    assert_eq!(m.pr_int(), 160);
}

#[test]
fn global_measurement_qrs_dur() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 260;
    m.qrs_offset = 360;
    assert_eq!(m.qrs_dur(), 100);
}

#[test]
fn global_measurement_set_qrs_dur() {
    let mut m = GlobalMeasurement::default();
    m.set_qrs_dur(100);
    assert_eq!(m.qrs_onset, 400); // default when unset
    assert_eq!(m.qrs_offset, 500);
    assert_eq!(m.qrs_dur(), 100);
}

#[test]
fn global_measurement_set_qrs_dur_with_existing_onset() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 200;
    m.set_qrs_dur(100);
    assert_eq!(m.qrs_onset, 200);
    assert_eq!(m.qrs_offset, 300);
}

#[test]
fn global_measurement_qt_dur() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 260;
    m.t_offset = 660;
    assert_eq!(m.qt_dur(), 400);
}

#[test]
fn global_measurement_set_qt_dur() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 260;
    m.set_qt_dur(400);
    assert_eq!(m.t_offset, 660);
    assert_eq!(m.qt_dur(), 400);
}

#[test]
fn global_measurement_set_qt_dur_no_qrs_onset() {
    let mut m = GlobalMeasurement::default();
    m.set_qt_dur(400);
    assert_eq!(m.t_offset, GlobalMeasurement::NO_VALUE);
}

#[test]
fn global_measurement_calc_qtc_bazett() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 200;
    m.t_offset = 600; // QT = 400ms
    let avg_rr = 800; // 800ms RR interval
    let hr = 75;
    let qtc = m.calc_qtc(avg_rr, hr, QTcCalcType::Bazett);
    // QTc = QT / sqrt(RR) = 400 / sqrt(0.8) ≈ 447
    assert!(qtc > 440 && qtc < 455, "Bazett QTc was {}", qtc);
}

#[test]
fn global_measurement_calc_qtc_fridericia() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 200;
    m.t_offset = 600; // QT = 400
    let qtc = m.calc_qtc(800, 75, QTcCalcType::Fridericia);
    // QTc = QT / RR^(1/3) = 400 / 0.8^(1/3) ≈ 430
    assert!(qtc > 425 && qtc < 440, "Fridericia QTc was {}", qtc);
}

#[test]
fn global_measurement_calc_qtc_framingham() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 200;
    m.t_offset = 600; // QT = 400
    let qtc = m.calc_qtc(800, 75, QTcCalcType::Framingham);
    // QTc = QT + 154*(1-RR) = 400 + 154*(1-0.8) = 400 + 30.8 ≈ 430
    assert!(qtc > 425 && qtc < 435, "Framingham QTc was {}", qtc);
}

#[test]
fn global_measurement_calc_qtc_hodges() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 200;
    m.t_offset = 600; // QT = 400
    let qtc = m.calc_qtc(800, 75, QTcCalcType::Hodges);
    // QTc = QT + 1.75*(HR-60) = 400 + 1.75*15 = 400 + 26.25 ≈ 426
    assert!(qtc > 420 && qtc < 430, "Hodges QTc was {}", qtc);
}

#[test]
fn global_measurement_calc_qtc_unknown() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 200;
    m.t_offset = 600;
    assert_eq!(m.calc_qtc(800, 75, QTcCalcType::Unknown), GlobalMeasurement::NO_VALUE);
}

#[test]
fn global_measurement_calc_qtc_no_rr() {
    let mut m = GlobalMeasurement::default();
    m.qrs_onset = 200;
    m.t_offset = 600;
    assert_eq!(m.calc_qtc(0, 75, QTcCalcType::Bazett), GlobalMeasurement::NO_VALUE);
    assert_eq!(m.calc_qtc(GlobalMeasurement::NO_VALUE, 75, QTcCalcType::Bazett), GlobalMeasurement::NO_VALUE);
}

#[test]
fn global_measurement_calc_qtc_no_qt() {
    let m = GlobalMeasurement::default(); // no t_offset set
    assert_eq!(m.calc_qtc(800, 75, QTcCalcType::Bazett), GlobalMeasurement::NO_VALUE);
}

// ── GlobalMeasurements ──────────────────────────────────────────────────

#[test]
fn global_measurements_default() {
    let gm = GlobalMeasurements::default();
    assert_eq!(gm.vent_rate(), 0); // NO_VALUE avg_rr → 0
    assert_eq!(gm.avg_rr, GlobalMeasurement::NO_VALUE);
    assert_eq!(gm.avg_pp, GlobalMeasurement::NO_VALUE);
    assert_eq!(gm.p_dur(), GlobalMeasurement::NO_VALUE);
    assert_eq!(gm.qrs_dur(), GlobalMeasurement::NO_VALUE);
}

#[test]
fn global_measurements_vent_rate_from_rr() {
    let mut gm = GlobalMeasurements::default();
    gm.avg_rr = 800; // 800ms → 75 bpm
    assert_eq!(gm.vent_rate(), 75);
}

#[test]
fn global_measurements_vent_rate_explicit() {
    let mut gm = GlobalMeasurements::default();
    gm.set_vent_rate(72);
    assert_eq!(gm.vent_rate(), 72);
}

#[test]
fn global_measurements_set_p_dur() {
    let mut gm = GlobalMeasurements::default();
    gm.set_p_dur(100);
    assert_eq!(gm.p_dur(), 100);
}

#[test]
fn global_measurements_set_qrs_dur() {
    let mut gm = GlobalMeasurements::default();
    gm.set_qrs_dur(120);
    assert_eq!(gm.qrs_dur(), 120);
}

#[test]
fn global_measurements_set_qt_dur() {
    let mut gm = GlobalMeasurements::default();
    gm.set_qrs_dur(100); // need QRS onset first
    gm.set_qt_dur(400);
    assert_eq!(gm.qt_dur(), 400);
}

#[test]
fn global_measurements_qtc_type() {
    let mut gm = GlobalMeasurements::default();
    // Default qtc is NO_VALUE → qtc_type maps diff=0 to Bazett
    assert_eq!(gm.qtc_type(), QTcCalcType::Bazett);

    gm.set_qtc_type(QTcCalcType::Hodges);
    assert_eq!(gm.qtc_type(), QTcCalcType::Hodges);

    gm.set_qtc_type(QTcCalcType::Fridericia);
    assert_eq!(gm.qtc_type(), QTcCalcType::Fridericia);

    gm.set_qtc_type(QTcCalcType::Framingham);
    assert_eq!(gm.qtc_type(), QTcCalcType::Framingham);

    // Setting explicit QTc value makes type Unknown
    gm.set_qtc(400);
    assert_eq!(gm.qtc_type(), QTcCalcType::Unknown);
}

#[test]
fn global_measurements_clone() {
    let mut gm = GlobalMeasurements::default();
    gm.avg_rr = 800;
    gm.set_vent_rate(75);
    gm.set_p_dur(80);
    let gm2 = gm.clone();
    assert_eq!(gm2.avg_rr, 800);
    assert_eq!(gm2.vent_rate(), 75);
    assert_eq!(gm2.p_dur(), 80);
}

// ── Spike ───────────────────────────────────────────────────────────────

#[test]
fn spike_default() {
    let s = Spike::default();
    assert_eq!(s.time, GlobalMeasurement::NO_VALUE);
    assert_eq!(s.amplitude, GlobalMeasurement::NO_AXIS_VALUE);
}

#[test]
fn spike_custom() {
    let s = Spike { time: 500, amplitude: -200 };
    assert_eq!(s.time, 500);
    assert_eq!(s.amplitude, -200);
}

// ── Morphology ──────────────────────────────────────────────────────────

#[test]
fn morphology_values() {
    assert_eq!(Morphology::Unknown as u8, 0);
    assert_eq!(Morphology::Positive as u8, 1);
    assert_eq!(Morphology::Negative as u8, 2);
    assert_eq!(Morphology::NotchedWShaped as u8, 8);
    assert_eq!(Morphology::default(), Morphology::Unknown);
}

// ── MeasurementType ─────────────────────────────────────────────────────

#[test]
fn measurement_type_values() {
    assert_eq!(MeasurementType::None as i32, -1);
    assert_eq!(MeasurementType::Pdur as i32, 0);
    assert_eq!(MeasurementType::PRint as i32, 1);
    assert_eq!(MeasurementType::QRSdur as i32, 2);
    assert_eq!(MeasurementType::Rnotch as i32, 43);
}

#[test]
fn measurement_type_aliases() {
    assert_eq!(MeasurementType::RONSET, MeasurementType::Qoffset);
    assert_eq!(MeasurementType::SONSET, MeasurementType::Roffset);
    assert_eq!(MeasurementType::RRONSET, MeasurementType::Soffset);
    assert_eq!(MeasurementType::SSONSET, MeasurementType::RRoffset);
    assert_eq!(MeasurementType::RRRONSET, MeasurementType::SSoffset);
}

// ── LeadMeasurement ─────────────────────────────────────────────────────

#[test]
fn lead_measurement_default() {
    let lm = LeadMeasurement::new();
    assert_eq!(lm.lead_type, LeadType::Unknown);
    assert_eq!(lm.count(), 0);
}

#[test]
fn lead_measurement_with_lead_type() {
    let lm = LeadMeasurement::with_lead_type(LeadType::V1);
    assert_eq!(lm.lead_type, LeadType::V1);
    assert_eq!(lm.count(), 0);
}

#[test]
fn lead_measurement_get_missing_returns_no_value() {
    let lm = LeadMeasurement::new();
    assert_eq!(lm.get(MeasurementType::Pdur), LeadMeasurement::NO_VALUE);
    assert_eq!(lm.get(MeasurementType::QRSdur), LeadMeasurement::NO_VALUE);
}

#[test]
fn lead_measurement_set_and_get() {
    let mut lm = LeadMeasurement::new();
    lm.set(MeasurementType::Pdur, 80);
    lm.set(MeasurementType::QRSdur, 100);
    lm.set(MeasurementType::Ramp, 1500);

    assert_eq!(lm.count(), 3);
    assert_eq!(lm.get(MeasurementType::Pdur), 80);
    assert_eq!(lm.get(MeasurementType::QRSdur), 100);
    assert_eq!(lm.get(MeasurementType::Ramp), 1500);
}

#[test]
fn lead_measurement_set_no_value_removes() {
    let mut lm = LeadMeasurement::new();
    lm.set(MeasurementType::Pdur, 80);
    assert_eq!(lm.count(), 1);

    lm.set(MeasurementType::Pdur, LeadMeasurement::NO_VALUE);
    assert_eq!(lm.count(), 0);
    assert_eq!(lm.get(MeasurementType::Pdur), LeadMeasurement::NO_VALUE);
}

#[test]
fn lead_measurement_overwrite() {
    let mut lm = LeadMeasurement::new();
    lm.set(MeasurementType::Pdur, 80);
    lm.set(MeasurementType::Pdur, 90);
    assert_eq!(lm.count(), 1);
    assert_eq!(lm.get(MeasurementType::Pdur), 90);
}

#[test]
fn lead_measurement_get_by_index() {
    let mut lm = LeadMeasurement::new();
    lm.set(MeasurementType::Pdur, 80);
    lm.set(MeasurementType::QRSdur, 100);

    // BTreeMap stores in key order; Pdur=0, QRSdur=2
    assert_eq!(lm.get_value_by_index(0), 80);
    assert_eq!(lm.get_value_by_index(1), 100);
    assert_eq!(lm.get_value_by_index(99), LeadMeasurement::NO_VALUE);
}

#[test]
fn lead_measurement_get_key_by_index() {
    let mut lm = LeadMeasurement::new();
    lm.set(MeasurementType::Pdur, 80);
    lm.set(MeasurementType::Ramp, 1500);

    assert_eq!(lm.get_key_by_index(0), MeasurementType::Pdur);
    assert_eq!(lm.get_key_by_index(1), MeasurementType::Ramp);
    assert_eq!(lm.get_key_by_index(99), MeasurementType::None);
}

#[test]
fn lead_measurement_negative_values() {
    let mut lm = LeadMeasurement::new();
    lm.set(MeasurementType::Samp, -500);
    assert_eq!(lm.get(MeasurementType::Samp), -500);
}

// ── LeadMeasurements ────────────────────────────────────────────────────

#[test]
fn lead_measurements_default() {
    let lms = LeadMeasurements::default();
    assert!(lms.measurements.is_none());
}

#[test]
fn lead_measurements_with_count() {
    let lms = LeadMeasurements::with_count(12);
    assert!(lms.measurements.is_some());
    assert_eq!(lms.measurements.as_ref().unwrap().len(), 12);
}
