// ECG format
//
// Abstract interface for ECG file format implementations.
// Original author: Maarten JB van Ettinger.

use std::io::{Read, Write};

use crate::converter::EcgConfig;
use crate::demographics::Demographic;
use crate::diagnostic::DiagnosticStatements;
use crate::measurements::{GlobalMeasurementProvider, LeadMeasurementProvider};
use crate::signal::Signals;

/// Trait for signal access.
pub trait SignalProvider {
    /// Get the signals of an ECG.
    fn get_signals(&self) -> Option<&Signals>;
    /// Set the signals of an ECG.
    fn set_signals(&mut self, signals: Signals) -> i32;
}

/// Abstract interface for ECG file format implementations.
pub trait EcgFormat: SignalProvider + GlobalMeasurementProvider + LeadMeasurementProvider {
    /// Standard anonymous single byte char.
    const STANDARD_ANONYMOUS: u8 = b'*';

    /// Get configuration for this format.
    fn config(&self) -> Option<&EcgConfig>;

    /// Check whether configuration works.
    fn configuration_works(&self) -> bool {
        self.config().map_or(true, |c| c.configuration_works())
    }

    /// Whether the stream can be closed after reading.
    fn can_close_stream(&self) -> bool {
        true
    }

    /// Whether this format supports BufferedStream.
    fn supports_buffered_stream(&self) -> bool {
        false
    }

    /// Read an ECG file from a reader.
    fn read_from(&mut self, input: &mut dyn Read, offset: usize) -> i32;

    /// Read an ECG file from a file path.
    fn read_file(&mut self, file: &str, offset: usize) -> i32;

    /// Read an ECG file from a byte buffer.
    fn read_bytes(&mut self, buffer: &[u8], offset: usize) -> i32;

    /// Write ECG format to a writer.
    fn write_to(&self, output: &mut dyn Write) -> i32;

    /// Write ECG format to a file path.
    fn write_file(&self, file: &str) -> i32;

    /// Write ECG format to a byte buffer.
    fn write_bytes(&self, buffer: &mut [u8], offset: usize) -> i32;

    /// Check if data matches this format.
    fn check_format_reader(&self, input: &mut dyn Read, offset: usize) -> bool;

    /// Check if file matches this format.
    fn check_format_file(&self, file: &str, offset: usize) -> bool;

    /// Check if buffer matches this format.
    fn check_format_bytes(&self, buffer: &[u8], offset: usize) -> bool;

    /// Get demographics accessor.
    fn demographics(&self) -> Option<&dyn Demographic>;

    /// Get mutable demographics accessor.
    fn demographics_mut(&mut self) -> Option<&mut dyn Demographic>;

    /// Get diagnostics accessor.
    fn diagnostics(&self) -> Option<&dyn DiagnosticStatements>;

    /// Get mutable diagnostics accessor.
    fn diagnostics_mut(&mut self) -> Option<&mut dyn DiagnosticStatements>;

    /// Anonymize the ECG file.
    fn anonymous(&mut self, anon_type: u8);

    /// Anonymize with standard char.
    fn anonymous_standard(&mut self) {
        self.anonymous(Self::STANDARD_ANONYMOUS);
    }

    /// Determine size of file.
    fn get_file_size(&self) -> usize;

    /// Check if format works.
    fn works(&self) -> bool;

    /// Empty the ECG file.
    fn empty(&mut self);
}
