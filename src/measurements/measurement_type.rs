// Measurement type
//
// Measurement type enumeration for lead measurements.
// Original author: Maarten JB van Ettinger.

/// Measurement type for lead measurements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum MeasurementType {
    #[default]
    None = -1,
    Pdur = 0,
    PRint = 1,
    QRSdur = 2,
    QTint = 3,
    Qdur = 4,
    Rdur = 5,
    Sdur = 6,
    RRdur = 7,
    SSdur = 8,
    RRRdur = 9,
    Qamp = 10,
    Ramp = 11,
    Samp = 12,
    RRamp = 13,
    SSamp = 14,
    RRRamp = 15,
    Jamp = 16,
    PampPos = 17,
    PampMin = 18,
    TampPos = 19,
    TampMin = 20,
    STslope = 21,
    Pmorphology = 22,
    Tmorphology = 23,
    IsoElectricQrsOnset = 24,
    IsoElectricQrsEnd = 25,
    IntrinsicoidDeflection = 26,
    QualityCode = 27,
    STampJ20 = 28,
    STampJ60 = 29,
    STampJ80 = 30,
    STamp1_16RR = 31,
    STamp1_8RR = 32,
    QRSonset = 33,
    QRSoffset = 34,
    Qoffset = 35,
    Roffset = 36,
    Soffset = 37,
    RRoffset = 38,
    SSoffset = 39,
    RRRoffset = 40,
    Toffset = 41,
    Pnotch = 42,
    Rnotch = 43,
}

impl MeasurementType {
    // Aliases matching C# source
    pub const RONSET: MeasurementType = MeasurementType::Qoffset;
    pub const SONSET: MeasurementType = MeasurementType::Roffset;
    pub const RRONSET: MeasurementType = MeasurementType::Soffset;
    pub const SSONSET: MeasurementType = MeasurementType::RRoffset;
    pub const RRRONSET: MeasurementType = MeasurementType::SSoffset;
}
