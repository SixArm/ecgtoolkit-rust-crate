// Tests for the signal module: Signal, Signals, QrsZone.

use ecgtoolkit::signal::*;
use ecgtoolkit::types::LeadType;

// ── QrsZone ─────────────────────────────────────────────────────────────

#[test]
fn qrs_zone_new() {
    let z = QrsZone::new(1, 100, 150, 200);
    assert_eq!(z.zone_type, 1);
    assert_eq!(z.start, 100);
    assert_eq!(z.fiducial, 150);
    assert_eq!(z.end, 200);
}

#[test]
fn qrs_zone_default() {
    let z = QrsZone::default();
    assert_eq!(z.zone_type, 0);
    assert_eq!(z.start, 0);
    assert_eq!(z.fiducial, 0);
    assert_eq!(z.end, 0);
}

#[test]
fn qrs_zone_clone() {
    let a = QrsZone::new(2, 50, 75, 100);
    let b = a.clone();
    assert_eq!(a.zone_type, b.zone_type);
    assert_eq!(a.start, b.start);
    assert_eq!(a.fiducial, b.fiducial);
    assert_eq!(a.end, b.end);
}

// ── Signal ──────────────────────────────────────────────────────────────

fn make_signal(lt: LeadType, start: i32, end: i32, data: Vec<i16>) -> Signal {
    Signal {
        lead_type: lt,
        rhythm_start: start,
        rhythm_end: end,
        rhythm: Some(data),
        median: None,
    }
}

#[test]
fn signal_default() {
    let s = Signal::default();
    assert_eq!(s.lead_type, LeadType::Unknown);
    assert_eq!(s.rhythm_start, 0);
    assert_eq!(s.rhythm_end, 0);
    assert!(s.rhythm.is_none());
    assert!(s.median.is_none());
}

#[test]
fn signal_clone() {
    let s = make_signal(LeadType::I, 0, 100, vec![1, 2, 3]);
    let s2 = s.clone();
    assert_eq!(s.lead_type, s2.lead_type);
    assert_eq!(s.rhythm, s2.rhythm);
}

#[test]
fn signal_is_normal_eight_standard_leads() {
    let leads: Vec<Signal> = (1..=8)
        .map(|i| make_signal(LeadType::from_u8(i), 0, 100, vec![0; 100]))
        .collect();
    assert!(Signal::is_normal(&leads));
}

#[test]
fn signal_is_normal_wrong_order() {
    let mut leads: Vec<Signal> = (1..=8)
        .map(|i| make_signal(LeadType::from_u8(i), 0, 100, vec![0; 100]))
        .collect();
    leads.swap(0, 1); // swap I and II
    assert!(!Signal::is_normal(&leads));
}

#[test]
fn signal_is_normal_too_few_leads() {
    let leads: Vec<Signal> = (1..=7)
        .map(|i| make_signal(LeadType::from_u8(i), 0, 100, vec![0; 100]))
        .collect();
    assert!(!Signal::is_normal(&leads));
}

#[test]
fn signal_is_normal_empty() {
    assert!(!Signal::is_normal(&[]));
}

#[test]
fn signal_nr_simultaneously_all_same_timing() {
    let leads: Vec<Signal> = (1..=4)
        .map(|i| make_signal(LeadType::from_u8(i), 0, 1000, vec![0; 1000]))
        .collect();
    assert_eq!(Signal::nr_simultaneously(&leads), 4);
}

#[test]
fn signal_nr_simultaneously_different_timing() {
    let leads = vec![
        make_signal(LeadType::I, 0, 1000, vec![0; 1000]),
        make_signal(LeadType::II, 0, 1000, vec![0; 1000]),
        make_signal(LeadType::V1, 100, 1100, vec![0; 1000]), // different start
    ];
    assert_eq!(Signal::nr_simultaneously(&leads), 2);
}

#[test]
fn signal_nr_simultaneously_within_tolerance() {
    let leads = vec![
        make_signal(LeadType::I, 0, 1000, vec![0; 1000]),
        make_signal(LeadType::II, 3, 1003, vec![0; 1000]), // within 5 tolerance
    ];
    assert_eq!(Signal::nr_simultaneously(&leads), 2);
}

#[test]
fn signal_nr_simultaneously_single_lead() {
    let leads = vec![make_signal(LeadType::I, 0, 1000, vec![0; 1000])];
    assert_eq!(Signal::nr_simultaneously(&leads), 0);
}

#[test]
fn signal_nr_simultaneously_empty() {
    assert_eq!(Signal::nr_simultaneously(&[]), 0);
}

#[test]
fn signal_sort_on_type() {
    let mut leads = vec![
        make_signal(LeadType::V6, 0, 100, vec![6]),
        make_signal(LeadType::I, 0, 100, vec![1]),
        make_signal(LeadType::V1, 0, 100, vec![3]),
        make_signal(LeadType::II, 0, 100, vec![2]),
    ];
    Signal::sort_on_type(&mut leads);
    assert_eq!(leads[0].lead_type, LeadType::I);
    assert_eq!(leads[1].lead_type, LeadType::II);
    assert_eq!(leads[2].lead_type, LeadType::V1);
    assert_eq!(leads[3].lead_type, LeadType::V6);
}

#[test]
fn signal_sort_on_type_already_sorted() {
    let mut leads: Vec<Signal> = (1..=4)
        .map(|i| make_signal(LeadType::from_u8(i), 0, 100, vec![i as i16]))
        .collect();
    Signal::sort_on_type(&mut leads);
    for (idx, lead) in leads.iter().enumerate() {
        assert_eq!(lead.lead_type, LeadType::from_u8((idx + 1) as u8));
    }
}

#[test]
fn signal_sort_on_type_single() {
    let mut leads = vec![make_signal(LeadType::V1, 0, 100, vec![1])];
    Signal::sort_on_type(&mut leads); // should not panic
    assert_eq!(leads[0].lead_type, LeadType::V1);
}

// ── Signals container ───────────────────────────────────────────────────

#[test]
fn signals_default() {
    let s = Signals::default();
    assert_eq!(s.nr_leads(), 0);
    assert!(s.get_leads().is_none());
}

#[test]
fn signals_with_nr_leads() {
    let s = Signals::with_nr_leads(8);
    assert_eq!(s.nr_leads(), 8);
    assert!(s.get_leads().is_some());
    assert_eq!(s.get_leads().unwrap().len(), 8);
}

#[test]
fn signals_set_and_get_leads() {
    let mut s = Signals::new();
    let leads = vec![
        make_signal(LeadType::I, 0, 100, vec![1, 2, 3]),
        make_signal(LeadType::II, 0, 100, vec![4, 5, 6]),
    ];
    s.set_leads(leads);
    assert_eq!(s.nr_leads(), 2);
    assert_eq!(s.get(0).unwrap().lead_type, LeadType::I);
    assert_eq!(s.get(1).unwrap().lead_type, LeadType::II);
}

#[test]
fn signals_get_out_of_bounds() {
    let s = Signals::with_nr_leads(2);
    assert!(s.get(0).is_some());
    assert!(s.get(1).is_some());
    assert!(s.get(2).is_none());
    assert!(s.get(100).is_none());
}

#[test]
fn signals_calculate_start_and_end() {
    let mut s = Signals::new();
    s.set_leads(vec![
        make_signal(LeadType::I, 10, 500, vec![0; 490]),
        make_signal(LeadType::II, 0, 600, vec![0; 600]),
        make_signal(LeadType::V1, 20, 400, vec![0; 380]),
    ]);
    let (start, end) = s.calculate_start_and_end();
    assert_eq!(start, 0);
    assert_eq!(end, 600);
}

#[test]
fn signals_calculate_start_and_end_no_leads() {
    let s = Signals::new();
    let (start, end) = s.calculate_start_and_end();
    assert_eq!(start, i32::MAX);
    assert_eq!(end, i32::MIN);
}

#[test]
fn signals_is_normal() {
    let mut s = Signals::new();
    let leads: Vec<Signal> = (1..=8)
        .map(|i| make_signal(LeadType::from_u8(i), 0, 100, vec![0; 100]))
        .collect();
    s.set_leads(leads);
    assert!(s.is_normal());
}

#[test]
fn signals_is_normal_false() {
    let s = Signals::new();
    assert!(!s.is_normal());
}

#[test]
fn signals_sort_on_type() {
    let mut s = Signals::new();
    s.set_leads(vec![
        make_signal(LeadType::V6, 0, 100, vec![0]),
        make_signal(LeadType::I, 0, 100, vec![0]),
        make_signal(LeadType::II, 0, 100, vec![0]),
    ]);
    s.sort_on_type();
    let leads = s.get_leads().unwrap();
    assert_eq!(leads[0].lead_type, LeadType::I);
    assert_eq!(leads[1].lead_type, LeadType::II);
    assert_eq!(leads[2].lead_type, LeadType::V6);
}

fn make_twelve_lead_signals() -> Signals {
    let twelve_types = [
        LeadType::I, LeadType::II, LeadType::III,
        LeadType::AVR, LeadType::AVL, LeadType::AVF,
        LeadType::V1, LeadType::V2, LeadType::V3,
        LeadType::V4, LeadType::V5, LeadType::V6,
    ];
    let mut s = Signals::new();
    let leads: Vec<Signal> = twelve_types.iter()
        .map(|&lt| make_signal(lt, 0, 1000, vec![0; 1000]))
        .collect();
    s.set_leads(leads);
    s.rhythm_avm = 1.0;
    s.rhythm_samples_per_second = 500;
    s
}

#[test]
fn signals_is_twelve_leads() {
    let s = make_twelve_lead_signals();
    assert!(s.is_twelve_leads());
}

#[test]
fn signals_is_twelve_leads_wrong_types() {
    let mut s = Signals::new();
    let leads: Vec<Signal> = (1..=12)
        .map(|i| make_signal(LeadType::from_u8(i), 0, 1000, vec![0; 1000]))
        .collect();
    s.set_leads(leads);
    // Leads 1-8 are I,II,V1..V6 but 9-12 are V7,V2R,V3R,V4R not III,aVR,aVL,aVF
    assert!(!s.is_twelve_leads());
}

#[test]
fn signals_is_twelve_leads_no_leads() {
    let s = Signals::new();
    assert!(!s.is_twelve_leads());
}

fn make_fifteen_lead_signals_variant1() -> Signals {
    let types = [
        LeadType::I, LeadType::II, LeadType::III,
        LeadType::AVR, LeadType::AVL, LeadType::AVF,
        LeadType::V1, LeadType::V2, LeadType::V3,
        LeadType::V4, LeadType::V5, LeadType::V6,
        LeadType::V3R, LeadType::V4R, LeadType::V7,
    ];
    let mut s = Signals::new();
    let leads: Vec<Signal> = types.iter()
        .map(|&lt| make_signal(lt, 0, 1000, vec![0; 1000]))
        .collect();
    s.set_leads(leads);
    s
}

fn make_fifteen_lead_signals_variant2() -> Signals {
    let types = [
        LeadType::I, LeadType::II, LeadType::III,
        LeadType::AVR, LeadType::AVL, LeadType::AVF,
        LeadType::V1, LeadType::V2, LeadType::V3,
        LeadType::V4, LeadType::V5, LeadType::V6,
        LeadType::V7, LeadType::V8, LeadType::V9,
    ];
    let mut s = Signals::new();
    let leads: Vec<Signal> = types.iter()
        .map(|&lt| make_signal(lt, 0, 1000, vec![0; 1000]))
        .collect();
    s.set_leads(leads);
    s
}

#[test]
fn signals_is_fifteen_leads_variant1() {
    let s = make_fifteen_lead_signals_variant1();
    assert!(s.is_fifteen_leads());
}

#[test]
fn signals_is_fifteen_leads_variant2() {
    let s = make_fifteen_lead_signals_variant2();
    assert!(s.is_fifteen_leads());
}

#[test]
fn signals_is_fifteen_leads_false_for_twelve() {
    let s = make_twelve_lead_signals();
    assert!(!s.is_fifteen_leads());
}

#[test]
fn signals_clone() {
    let mut s = make_twelve_lead_signals();
    s.qrs_zone = Some(vec![QrsZone::new(1, 10, 20, 30)]);
    let s2 = s.clone();
    assert_eq!(s2.nr_leads(), 12);
    assert_eq!(s2.rhythm_avm, s.rhythm_avm);
    assert_eq!(s2.qrs_zone.as_ref().unwrap().len(), 1);
}

#[test]
fn signals_set_avm() {
    let mut s = Signals::new();
    s.rhythm_avm = 2.5;
    s.median_avm = 2.5;
    let leads = vec![
        Signal {
            lead_type: LeadType::I,
            rhythm_start: 0,
            rhythm_end: 4,
            rhythm: Some(vec![100, 200, -100, -200]),
            median: Some(vec![50, 100, -50, -100]),
        },
    ];
    s.set_leads(leads);
    s.set_avm(5.0);
    assert_eq!(s.rhythm_avm, 5.0);
    assert_eq!(s.median_avm, 5.0);
    // Values should be halved: (val * 2.5) / 5.0 = val / 2
    let rhythm = s.get(0).unwrap().rhythm.as_ref().unwrap();
    assert_eq!(rhythm[0], 50);
    assert_eq!(rhythm[1], 100);
    assert_eq!(rhythm[2], -50);
    assert_eq!(rhythm[3], -100);
}

#[test]
fn signals_set_avm_zero_is_noop() {
    let mut s = Signals::new();
    s.rhythm_avm = 2.5;
    s.set_leads(vec![Signal {
        lead_type: LeadType::I,
        rhythm_start: 0,
        rhythm_end: 2,
        rhythm: Some(vec![100, 200]),
        median: None,
    }]);
    s.set_avm(0.0);
    assert_eq!(s.rhythm_avm, 2.5); // unchanged
    let rhythm = s.get(0).unwrap().rhythm.as_ref().unwrap();
    assert_eq!(rhythm[0], 100); // unchanged
}

#[test]
fn signals_calculate_twelve_leads_from_eight() {
    // Create 8 leads: I, II, V1-V6 with known data
    let eight_types = [
        LeadType::I, LeadType::II,
        LeadType::V1, LeadType::V2, LeadType::V3,
        LeadType::V4, LeadType::V5, LeadType::V6,
    ];

    let mut s = Signals::new();
    s.rhythm_avm = 1.0;
    s.rhythm_samples_per_second = 500;

    let leads: Vec<Signal> = eight_types.iter()
        .enumerate()
        .map(|(idx, &lt)| {
            let val = (idx as i16 + 1) * 100;
            make_signal(lt, 0, 5, vec![val; 5])
        })
        .collect();
    s.set_leads(leads);

    let result = s.calculate_twelve_leads();
    assert!(result.is_some());

    let twelve = result.unwrap();
    assert_eq!(twelve.nr_leads(), 12);

    let leads = twelve.get_leads().unwrap();
    // Verify lead types
    assert_eq!(leads[0].lead_type, LeadType::I);
    assert_eq!(leads[1].lead_type, LeadType::II);
    assert_eq!(leads[2].lead_type, LeadType::III);
    assert_eq!(leads[3].lead_type, LeadType::AVR);
    assert_eq!(leads[4].lead_type, LeadType::AVL);
    assert_eq!(leads[5].lead_type, LeadType::AVF);

    // Verify calculated values: III = II - I = 200 - 100 = 100
    let iii = leads[2].rhythm.as_ref().unwrap();
    assert_eq!(iii[0], 100);

    // aVR = -(I + II) / 2 = -(100 + 200) / 2 = -150
    let avr = leads[3].rhythm.as_ref().unwrap();
    assert_eq!(avr[0], -150);

    // aVL = (2*I - II) / 2 = (200 - 200) / 2 = 0
    let avl = leads[4].rhythm.as_ref().unwrap();
    assert_eq!(avl[0], 0);

    // aVF = (2*II - I) / 2 = (400 - 100) / 2 = 150
    let avf = leads[5].rhythm.as_ref().unwrap();
    assert_eq!(avf[0], 150);
}

#[test]
fn signals_apply_lowpass_filter() {
    let mut s = Signals::new();
    s.rhythm_avm = 1.0;
    s.rhythm_samples_per_second = 500;
    // Create a simple signal
    let data: Vec<i16> = (0..500).map(|i| ((i as f64 * 0.1).sin() * 1000.0) as i16).collect();
    s.set_leads(vec![Signal {
        lead_type: LeadType::I,
        rhythm_start: 0,
        rhythm_end: 500,
        rhythm: Some(data),
        median: None,
    }]);

    let filtered = s.apply_lowpass_filter(50.0);
    assert!(filtered.is_some());
    let f = filtered.unwrap();
    assert_eq!(f.nr_leads(), 1);
    assert!(f.get(0).unwrap().rhythm.is_some());
    assert_eq!(f.get(0).unwrap().rhythm.as_ref().unwrap().len(), 500);
}

#[test]
fn signals_apply_highpass_filter() {
    let mut s = Signals::new();
    s.rhythm_avm = 1.0;
    s.rhythm_samples_per_second = 500;
    let data: Vec<i16> = (0..500).map(|i| ((i as f64 * 0.5).sin() * 500.0) as i16).collect();
    s.set_leads(vec![Signal {
        lead_type: LeadType::I,
        rhythm_start: 0,
        rhythm_end: 500,
        rhythm: Some(data),
        median: None,
    }]);

    let filtered = s.apply_highpass_filter(0.5);
    assert!(filtered.is_some());
}

#[test]
fn signals_apply_bandpass_filter() {
    let mut s = Signals::new();
    s.rhythm_avm = 1.0;
    s.rhythm_samples_per_second = 500;
    let data: Vec<i16> = (0..500).map(|i| ((i as f64 * 0.3).sin() * 800.0) as i16).collect();
    s.set_leads(vec![Signal {
        lead_type: LeadType::I,
        rhythm_start: 0,
        rhythm_end: 500,
        rhythm: Some(data),
        median: None,
    }]);

    let filtered = s.apply_bandpass_filter(0.5, 40.0);
    assert!(filtered.is_some());
}

#[test]
fn signals_filter_no_leads_returns_some_empty() {
    let s = Signals::new();
    let filtered = s.apply_lowpass_filter(50.0);
    // No leads means the filter loop is skipped, returns Some with no leads
    assert!(filtered.is_some());
    assert_eq!(filtered.unwrap().nr_leads(), 0);
}

#[test]
fn signals_nr_simultaneously() {
    let s = make_twelve_lead_signals();
    assert_eq!(s.nr_simultaneously(), 12);
}
