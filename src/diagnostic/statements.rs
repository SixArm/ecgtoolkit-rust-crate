// Diagnostic statements
//
// Diagnostic statement data for ECG records.
// Original author: Maarten JB van Ettinger.

/// Diagnostic statements for an ECG.
#[derive(Clone, Debug, Default)]
pub struct Statements {
    pub confirmed: bool,
    pub interpreted: bool,
    /// Time of diagnostic (Unix timestamp or 0).
    pub time: u64,
    pub statement: Option<Vec<String>>,
}

/// Interface for manipulation of diagnostic statements.
pub trait DiagnosticStatements {
    /// Get the diagnostic statements.
    fn get_diagnostic_statements(&self) -> Option<&Statements>;
    /// Set the diagnostic statements.
    fn set_diagnostic_statements(&mut self, stat: Statements) -> i32;
}
