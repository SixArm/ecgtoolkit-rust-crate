// Tests for the demographics module.

use ecgtoolkit::demographics::*;

// ── Drug ────────────────────────────────────────────────────────────────

#[test]
fn drug_default() {
    let d = Drug::default();
    assert_eq!(d.drug_class, 0);
    assert_eq!(d.class_code, 0);
    assert!(d.text_description.is_none());
}

#[test]
fn drug_with_values() {
    let d = Drug {
        drug_class: 1,
        class_code: 42,
        text_description: Some("Aspirin".to_string()),
    };
    assert_eq!(d.drug_class, 1);
    assert_eq!(d.class_code, 42);
    assert_eq!(d.text_description.as_deref(), Some("Aspirin"));
}

#[test]
fn drug_clone() {
    let d1 = Drug {
        drug_class: 3,
        class_code: 7,
        text_description: Some("Beta blocker".to_string()),
    };
    let d2 = d1.clone();
    assert_eq!(d1.drug_class, d2.drug_class);
    assert_eq!(d1.text_description, d2.text_description);
}

// ── AcquiringDeviceId ───────────────────────────────────────────────────

#[test]
fn acquiring_device_id_default() {
    let id = AcquiringDeviceId::default();
    assert_eq!(id.institution_nr, 0);
    assert_eq!(id.department_nr, 0);
    assert_eq!(id.device_id, 0);
    assert_eq!(id.device_type, 0);
    assert_eq!(id.manufacturer_id, 0);
    assert_eq!(id.device_capabilities, 0);
    assert_eq!(id.ac_frequency_environment, 0);
    assert_eq!(id.model_description, [0u8; 6]);
}

#[test]
fn acquiring_device_id_no_device() {
    let id = AcquiringDeviceId::no_device();
    assert_eq!(id.institution_nr, 0);
    assert_eq!(id.department_nr, 11);
    assert_eq!(id.device_id, 51);
    assert_eq!(id.device_type, 1); // System
    assert_eq!(id.device_capabilities, 0x8);
    assert_eq!(id.ac_frequency_environment, 1);
    assert_eq!(&id.model_description[..5], b"MCONV");
    assert_eq!(id.model_description[5], 0);
}

#[test]
fn acquiring_device_id_clone() {
    let id1 = AcquiringDeviceId::no_device();
    let id2 = id1.clone();
    assert_eq!(id1.department_nr, id2.department_nr);
    assert_eq!(id1.model_description, id2.model_description);
}
