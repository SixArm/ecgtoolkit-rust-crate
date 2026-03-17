// Tests for the diagnostic module.

use ecgtoolkit::diagnostic::*;

#[test]
fn statements_default() {
    let s = Statements::default();
    assert!(!s.confirmed);
    assert!(!s.interpreted);
    assert_eq!(s.time, 0);
    assert!(s.statement.is_none());
}

#[test]
fn statements_with_data() {
    let s = Statements {
        confirmed: true,
        interpreted: true,
        time: 1700000000,
        statement: Some(vec![
            "Normal sinus rhythm".to_string(),
            "Normal ECG".to_string(),
        ]),
    };
    assert!(s.confirmed);
    assert!(s.interpreted);
    assert_eq!(s.statement.as_ref().unwrap().len(), 2);
    assert_eq!(s.statement.as_ref().unwrap()[0], "Normal sinus rhythm");
}

#[test]
fn statements_clone() {
    let s1 = Statements {
        confirmed: true,
        interpreted: false,
        time: 12345,
        statement: Some(vec!["Test".to_string()]),
    };
    let s2 = s1.clone();
    assert_eq!(s1.confirmed, s2.confirmed);
    assert_eq!(s1.interpreted, s2.interpreted);
    assert_eq!(s1.time, s2.time);
    assert_eq!(s1.statement, s2.statement);
}
