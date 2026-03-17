// Demographic enumerations
//
// Enumerations for demographic data used in ECG records.
// Original author: Maarten JB van Ettinger.

/// Sex of the patient.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Sex {
    #[default]
    Unspecified = 0,
    Male = 1,
    Female = 2,
    Null = 0xff,
}

/// Race of the patient.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Race {
    #[default]
    Unspecified = 0,
    Caucasian = 1,
    Black = 2,
    Oriental = 3,
    Null = 0xff,
}

/// Definition of age units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum AgeDefinition {
    #[default]
    Unspecified = 0,
    Years = 1,
    Months = 2,
    Weeks = 3,
    Days = 4,
    Hours = 5,
}

/// Definition of height units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum HeightDefinition {
    #[default]
    Unspecified = 0,
    Centimeters = 1,
    Inches = 2,
    Millimeters = 3,
}

/// Definition of weight units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum WeightDefinition {
    #[default]
    Unspecified = 0,
    Kilogram = 1,
    Gram = 2,
    Pound = 3,
    Ounce = 4,
}

/// Device type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceType {
    #[default]
    Cart = 0,
    System = 1,
}

/// Device manufacturer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceManufacturer {
    #[default]
    Unknown = 0,
    Burdick = 1,
    Cambridge = 2,
    Compumed = 3,
    Datamed = 4,
    Fukuda = 5,
    HewlettPackard = 6,
    MarquetteElectronics = 7,
    MortaraInstruments = 8,
    NihonKohden = 9,
    Okin = 10,
    Quintin = 11,
    Siemens = 12,
    SpaceLabs = 13,
    Telemed = 14,
    Hellige = 15,
    Esaote = 16,
    Schiller = 17,
    PickerSchwarzer = 18,
    ElettronicTrentina = 19,
    Zwonitz = 20,
    Other = 100,
}

/// Electrode configuration codes for 12-lead ECG.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ElectrodeConfigCodeTwelveLead {
    #[default]
    Unspecified = 0,
    StandardTwelveLead = 1,
    MasonLikarAndIndividual = 2,
    MasonLikarAndPadded = 3,
    AllLeadPadded = 4,
    TwelveLeadDerivedXyz = 5,
    TwelveLeadDerivedNonStandard = 6,
}

/// Electrode configuration codes for XYZ ECG.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum ElectrodeConfigCodeXyz {
    #[default]
    Unspecified = 0,
    Frank = 1,
    McFeeParungao = 2,
    Cube = 3,
    BipolarUncorrected = 4,
    PseudoOrthogonal = 5,
    XyzDerivedTwelveLead = 6,
}
