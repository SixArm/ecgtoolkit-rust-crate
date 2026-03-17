// Morphology
//
// Morphology enumeration for lead measurements.
// Original author: Maarten JB van Ettinger.

/// Morphology for lead measurements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Morphology {
    #[default]
    Unknown = 0,
    Positive = 1,
    Negative = 2,
    PositiveNegative = 3,
    NegativePositive = 4,
    PositiveNegativePositive = 5,
    NegativePositiveNegative = 6,
    NotchedMShaped = 7,
    NotchedWShaped = 8,
}
