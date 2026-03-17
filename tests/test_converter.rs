// Tests for the converter module: EcgConfig.

use ecgtoolkit::converter::*;

// ── EcgConfig ───────────────────────────────────────────────────────────

#[test]
fn ecg_config_new_empty() {
    let cfg = EcgConfig::new(&[], &[], None);
    assert_eq!(cfg.nr_config_items(), 0);
    assert!(cfg.configuration_works());
}

#[test]
fn ecg_config_mandatory_items() {
    let cfg = EcgConfig::new(&["rate", "channels"], &["format"], None);
    assert_eq!(cfg.nr_config_items(), 3);
    assert!(!cfg.configuration_works()); // mandatory not set
}

#[test]
fn ecg_config_set_mandatory() {
    let mut cfg = EcgConfig::new(&["rate", "channels"], &[], None);
    assert!(!cfg.configuration_works());

    cfg.set("rate", Some("500"));
    assert!(!cfg.configuration_works()); // only one set

    cfg.set("channels", Some("12"));
    assert!(cfg.configuration_works()); // both set
}

#[test]
fn ecg_config_get_and_set() {
    let mut cfg = EcgConfig::new(&[], &["key1", "key2"], None);
    assert!(cfg.get("key1").is_none());

    cfg.set("key1", Some("value1"));
    assert_eq!(cfg.get("key1"), Some("value1"));

    cfg.set("key1", Some("updated"));
    assert_eq!(cfg.get("key1"), Some("updated"));
}

#[test]
fn ecg_config_set_none_removes() {
    let mut cfg = EcgConfig::new(&[], &["key1"], None);
    cfg.set("key1", Some("value"));
    assert_eq!(cfg.get("key1"), Some("value"));

    cfg.set("key1", None);
    assert!(cfg.get("key1").is_none());
}

#[test]
fn ecg_config_set_empty_string_removes() {
    let mut cfg = EcgConfig::new(&[], &["key1"], None);
    cfg.set("key1", Some("value"));
    cfg.set("key1", Some(""));
    assert!(cfg.get("key1").is_none());
}

#[test]
fn ecg_config_set_invalid_key_ignored() {
    let mut cfg = EcgConfig::new(&[], &["valid"], None);
    cfg.set("invalid", Some("value"));
    assert!(cfg.get("invalid").is_none());
}

#[test]
fn ecg_config_is_part_of_config() {
    let cfg = EcgConfig::new(&["must1"], &["opt1", "opt2"], None);
    assert!(cfg.is_part_of_config("must1"));
    assert!(cfg.is_part_of_config("opt1"));
    assert!(cfg.is_part_of_config("opt2"));
    assert!(!cfg.is_part_of_config("unknown"));
}

#[test]
fn ecg_config_get_by_index() {
    let cfg = EcgConfig::new(&["first"], &["second"], None);
    assert_eq!(cfg.get_by_index(0), Some("first"));
    assert_eq!(cfg.get_by_index(1), Some("second"));
    assert_eq!(cfg.get_by_index(2), None);
}

#[test]
fn ecg_config_get_config_item() {
    let cfg = EcgConfig::new(&["must1"], &["opt1"], None);
    let (name, must) = cfg.get_config_item(0).unwrap();
    assert_eq!(name, "must1");
    assert!(must);

    let (name, must) = cfg.get_config_item(1).unwrap();
    assert_eq!(name, "opt1");
    assert!(!must);

    assert!(cfg.get_config_item(2).is_none());
}

#[test]
fn ecg_config_new_all_must() {
    let cfg = EcgConfig::new_all(&["a", "b", "c"], true, None);
    assert_eq!(cfg.nr_config_items(), 3);
    assert!(!cfg.configuration_works());
}

#[test]
fn ecg_config_new_all_optional() {
    let cfg = EcgConfig::new_all(&["a", "b", "c"], false, None);
    assert_eq!(cfg.nr_config_items(), 3);
    assert!(cfg.configuration_works()); // all optional
}

#[test]
fn ecg_config_with_check_function() {
    fn always_fail() -> bool { false }

    let mut cfg = EcgConfig::new(&[], &["x"], Some(always_fail));
    cfg.set("x", Some("val"));
    assert!(!cfg.configuration_works()); // check function returns false
}

#[test]
fn ecg_config_with_check_function_pass() {
    fn always_pass() -> bool { true }

    let cfg = EcgConfig::new(&[], &[], Some(always_pass));
    assert!(cfg.configuration_works());
}

#[test]
fn ecg_config_set_from_same_structure() {
    let mut cfg1 = EcgConfig::new(&["a"], &["b"], None);
    cfg1.set("a", Some("1"));
    cfg1.set("b", Some("2"));

    let mut cfg2 = EcgConfig::new(&["a"], &["b"], None);
    assert!(cfg2.set_from(&cfg1));
    assert_eq!(cfg2.get("a"), Some("1"));
    assert_eq!(cfg2.get("b"), Some("2"));
}

#[test]
fn ecg_config_set_from_different_structure() {
    let cfg1 = EcgConfig::new(&["a"], &[], None);
    let mut cfg2 = EcgConfig::new(&["x"], &[], None);
    assert!(!cfg2.set_from(&cfg1));
}

#[test]
fn ecg_config_set_from_different_count() {
    let cfg1 = EcgConfig::new(&["a", "b"], &[], None);
    let mut cfg2 = EcgConfig::new(&["a"], &[], None);
    assert!(!cfg2.set_from(&cfg1));
}
