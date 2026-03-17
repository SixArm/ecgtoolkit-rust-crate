// Tests for the types module: LeadType, Date, and demographic enums.

use ecgtoolkit::types::*;

// ── LeadType ────────────────────────────────────────────────────────────

#[test]
fn lead_type_default_is_unknown() {
    assert_eq!(LeadType::default(), LeadType::Unknown);
}

#[test]
fn lead_type_from_u8_known_values() {
    assert_eq!(LeadType::from_u8(0), LeadType::Unknown);
    assert_eq!(LeadType::from_u8(1), LeadType::I);
    assert_eq!(LeadType::from_u8(2), LeadType::II);
    assert_eq!(LeadType::from_u8(3), LeadType::V1);
    assert_eq!(LeadType::from_u8(4), LeadType::V2);
    assert_eq!(LeadType::from_u8(5), LeadType::V3);
    assert_eq!(LeadType::from_u8(6), LeadType::V4);
    assert_eq!(LeadType::from_u8(7), LeadType::V5);
    assert_eq!(LeadType::from_u8(8), LeadType::V6);
}

#[test]
fn lead_type_from_u8_out_of_range_returns_unknown() {
    assert_eq!(LeadType::from_u8(255), LeadType::Unknown);
    assert_eq!(LeadType::from_u8(254), LeadType::Unknown);
}

#[test]
fn lead_type_ordering() {
    assert!(LeadType::Unknown < LeadType::I);
    assert!(LeadType::I < LeadType::II);
    assert!(LeadType::II < LeadType::V1);
}

#[test]
fn lead_type_clone_and_eq() {
    let a = LeadType::V3;
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn lead_type_round_trip() {
    for val in 0..=LeadType::DV10 as u8 {
        let lt = LeadType::from_u8(val);
        assert_eq!(lt as u8, val);
    }
}

// ── LeadTypeVitalRefId ──────────────────────────────────────────────────

#[test]
fn vital_ref_id_values() {
    assert_eq!(LeadTypeVitalRefId::MdcEcgLeadConfig as u16, 0);
    assert_eq!(LeadTypeVitalRefId::MdcEcgLeadI as u16, 1);
    assert_eq!(LeadTypeVitalRefId::MdcEcgLeadIII as u16, 61);
    assert_eq!(LeadTypeVitalRefId::MdcEcgLeadC as u16, 86);
    assert_eq!(LeadTypeVitalRefId::MdcEcgLeadCM7 as u16, 121);
    assert_eq!(LeadTypeVitalRefId::MdcEcgLeadRL as u16, 147);
    assert_eq!(LeadTypeVitalRefId::MdcEcgLeadV10 as u16, 151);
}

// ── Date ────────────────────────────────────────────────────────────────

#[test]
fn date_new() {
    let d = Date::new(2024, 3, 15);
    assert_eq!(d.year, 2024);
    assert_eq!(d.month, 3);
    assert_eq!(d.day, 15);
}

#[test]
fn date_default() {
    let d = Date::default();
    assert_eq!(d.year, 0);
    assert_eq!(d.month, 0);
    assert_eq!(d.day, 0);
    assert!(!d.is_existing_date());
}

#[test]
fn date_valid_dates() {
    assert!(Date::new(2024, 1, 1).is_existing_date());
    assert!(Date::new(2024, 1, 31).is_existing_date());
    assert!(Date::new(2024, 12, 31).is_existing_date());
    assert!(Date::new(1900, 6, 15).is_existing_date());
}

#[test]
fn date_leap_year() {
    // Divisible by 4, not by 100 → leap
    assert!(Date::new(2024, 2, 29).is_existing_date());
    assert!(Date::new(2020, 2, 29).is_existing_date());
    // Divisible by 100 but not 400 → not leap
    assert!(!Date::new(1900, 2, 29).is_existing_date());
    assert!(!Date::new(2100, 2, 29).is_existing_date());
    // Divisible by 400 → leap
    assert!(Date::new(2000, 2, 29).is_existing_date());
    assert!(Date::new(2400, 2, 29).is_existing_date());
}

#[test]
fn date_invalid_dates() {
    assert!(!Date::new(0, 1, 1).is_existing_date());     // year 0
    assert!(!Date::new(2024, 0, 1).is_existing_date());   // month 0
    assert!(!Date::new(2024, 13, 1).is_existing_date());  // month 13
    assert!(!Date::new(2024, 1, 0).is_existing_date());   // day 0
    assert!(!Date::new(2024, 1, 32).is_existing_date());  // day 32 Jan
    assert!(!Date::new(2023, 2, 29).is_existing_date());  // Feb 29 non-leap
    assert!(!Date::new(2024, 4, 31).is_existing_date());  // Apr 31
    assert!(!Date::new(2024, 6, 31).is_existing_date());  // Jun 31
    assert!(!Date::new(2024, 9, 31).is_existing_date());  // Sep 31
    assert!(!Date::new(2024, 11, 31).is_existing_date()); // Nov 31
}

#[test]
fn date_boundary_days_per_month() {
    // Last valid day of each month in a non-leap year
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (i, &max_day) in days.iter().enumerate() {
        let month = (i + 1) as u8;
        assert!(Date::new(2023, month, max_day).is_existing_date());
        assert!(!Date::new(2023, month, max_day + 1).is_existing_date());
    }
}

#[test]
fn date_clone_and_eq() {
    let a = Date::new(2024, 6, 15);
    let b = a.clone();
    assert_eq!(a, b);
}

// ── Demographic Enums ───────────────────────────────────────────────────

#[test]
fn sex_values() {
    assert_eq!(Sex::Unspecified as u8, 0);
    assert_eq!(Sex::Male as u8, 1);
    assert_eq!(Sex::Female as u8, 2);
    assert_eq!(Sex::Null as u8, 0xff);
    assert_eq!(Sex::default(), Sex::Unspecified);
}

#[test]
fn race_values() {
    assert_eq!(Race::Unspecified as u8, 0);
    assert_eq!(Race::Caucasian as u8, 1);
    assert_eq!(Race::Black as u8, 2);
    assert_eq!(Race::Oriental as u8, 3);
    assert_eq!(Race::Null as u8, 0xff);
    assert_eq!(Race::default(), Race::Unspecified);
}

#[test]
fn age_definition_values() {
    assert_eq!(AgeDefinition::Unspecified as u8, 0);
    assert_eq!(AgeDefinition::Years as u8, 1);
    assert_eq!(AgeDefinition::Months as u8, 2);
    assert_eq!(AgeDefinition::Hours as u8, 5);
}

#[test]
fn height_definition_values() {
    assert_eq!(HeightDefinition::Unspecified as u8, 0);
    assert_eq!(HeightDefinition::Centimeters as u8, 1);
    assert_eq!(HeightDefinition::Inches as u8, 2);
    assert_eq!(HeightDefinition::Millimeters as u8, 3);
}

#[test]
fn weight_definition_values() {
    assert_eq!(WeightDefinition::Unspecified as u8, 0);
    assert_eq!(WeightDefinition::Kilogram as u8, 1);
    assert_eq!(WeightDefinition::Ounce as u8, 4);
}

#[test]
fn device_type_values() {
    assert_eq!(DeviceType::Cart as u8, 0);
    assert_eq!(DeviceType::System as u8, 1);
}

#[test]
fn device_manufacturer_values() {
    assert_eq!(DeviceManufacturer::Unknown as u8, 0);
    assert_eq!(DeviceManufacturer::Burdick as u8, 1);
    assert_eq!(DeviceManufacturer::NihonKohden as u8, 9);
    assert_eq!(DeviceManufacturer::Other as u8, 100);
}

#[test]
fn electrode_config_twelve_lead_values() {
    assert_eq!(ElectrodeConfigCodeTwelveLead::Unspecified as u8, 0);
    assert_eq!(ElectrodeConfigCodeTwelveLead::StandardTwelveLead as u8, 1);
    assert_eq!(ElectrodeConfigCodeTwelveLead::TwelveLeadDerivedNonStandard as u8, 6);
}

#[test]
fn electrode_config_xyz_values() {
    assert_eq!(ElectrodeConfigCodeXyz::Unspecified as u8, 0);
    assert_eq!(ElectrodeConfigCodeXyz::Frank as u8, 1);
    assert_eq!(ElectrodeConfigCodeXyz::XyzDerivedTwelveLead as u8, 6);
}
