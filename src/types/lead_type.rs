// Lead type enumeration
//
// Enumeration for ECG lead types.
// Original author: Maarten JB van Ettinger.

/// Enumeration for lead types.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LeadType {
    #[default]
    Unknown = 0,
    I, II, V1, V2, V3, V4, V5, V6, V7,
    V2R, V3R, V4R, V5R, V6R, V7R,
    X, Y, Z,
    CC5, CM5, LA, RA, LL,
    FI, FE, FC, FA, FM, FF, FH,
    DI, DII, DV1, DV2, DV3, DV4, DV5, DV6,
    DV7, DV2R, DV3R, DV4R, DV5R, DV6R, DV7R,
    DX, DY, DZ, DCC5, DCM5, DLA, DRA, DLL,
    DfI, DfE, DfC, DfA, DfM, DfF, DfH,
    III, AVR, AVL, AVF, AVRneg,
    V8, V9, V8R, V9R,
    D, A, J,
    Defib, Extern,
    A1, A2, A3, A4,
    DV8, DV9, DV8R, DV9R, DD, DA, DJ,
    Chest, VLead, VR, VL, VF,
    MCL, MCL1, MCL2, MCL3, MCL4, MCL5, MCL6,
    CC, CC1, CC2, CC3, CC4, CC6, CC7,
    CM, CM1, CM2, CM3, CM4, CM6,
    DIII, DAVR, DAVL, DAVF, DAVRneg,
    DChest, DV, DVR, DVL, DVF,
    CM7, CH5, CS5, CB5, CR5, ML,
    AB1, AB2, AB3, AB4,
    ES, AS, AI, S,
    DDefib, DExtern, DA1, DA2, DA3, DA4,
    DMCL1, DMCL2, DMCL3, DMCL4, DMCL5, DMCL6,
    RL, CV5RL, CV6LL, CV6LU, V10,
    DMCL, DCC, DCC1, DCC2, DCC3, DCC4, DCC6,
    DCC7, DCM, DCM1, DCM2, DCM3, DCM4, DCM6,
    DCM7, DCH5, DCS5, DCB5, DCR5, DML,
    DAB1, DAB2, DAB3, DAB4,
    DES, DAS, DAI, DS, DRL,
    DCV5RL, DCV6LL, DCV6LU, DV10,
}

impl LeadType {
    /// Return the lead type from a u8 value.
    pub fn from_u8(val: u8) -> Self {
        // Safety: we check bounds
        if val <= LeadType::DV10 as u8 {
            // SAFETY: all values 0..=max are valid enum variants
            unsafe { std::mem::transmute(val) }
        } else {
            LeadType::Unknown
        }
    }
}

/// Enumeration for lead type vital reference IDs (MDC codes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LeadTypeVitalRefId {
    MdcEcgLeadConfig = 0,
    MdcEcgLeadI,
    MdcEcgLeadII,
    MdcEcgLeadV1,
    MdcEcgLeadV2,
    MdcEcgLeadV3,
    MdcEcgLeadV4,
    MdcEcgLeadV5,
    MdcEcgLeadV6,
    MdcEcgLeadV7,
    MdcEcgLeadV2R,
    MdcEcgLeadV3R,
    MdcEcgLeadV4R,
    MdcEcgLeadV5R,
    MdcEcgLeadV6R,
    MdcEcgLeadV7R,
    MdcEcgLeadX,
    MdcEcgLeadY,
    MdcEcgLeadZ,
    MdcEcgLeadCC5,
    MdcEcgLeadCM5,
    MdcEcgLeadLA,
    MdcEcgLeadRA,
    MdcEcgLeadLL,
    MdcEcgLeadFI,
    MdcEcgLeadFE,
    MdcEcgLeadFC,
    MdcEcgLeadFA,
    MdcEcgLeadFM,
    MdcEcgLeadFF,
    MdcEcgLeadFH,
    MdcEcgLeadDI,
    MdcEcgLeadDII,
    MdcEcgLeadDV1,
    MdcEcgLeadDV2,
    MdcEcgLeadDV3,
    MdcEcgLeadDV4,
    MdcEcgLeadDV5,
    MdcEcgLeadDV6,
    // Gap: 39..=60 not assigned
    MdcEcgLeadIII = 61,
    MdcEcgLeadAVR,
    MdcEcgLeadAVL,
    MdcEcgLeadAVF,
    MdcEcgLeadAVRneg,
    MdcEcgLeadV8,
    MdcEcgLeadV9,
    MdcEcgLeadV8R,
    MdcEcgLeadV9R,
    MdcEcgLeadD,
    MdcEcgLeadA,
    MdcEcgLeadJ,
    MdcEcgLeadDefib,
    MdcEcgLeadExtern,
    MdcEcgLeadA1,
    MdcEcgLeadA2,
    MdcEcgLeadA3,
    MdcEcgLeadA4,
    // Gap: 79..=85 not assigned
    MdcEcgLeadC = 86,
    MdcEcgLeadV,
    MdcEcgLeadVR,
    MdcEcgLeadVL,
    MdcEcgLeadVF,
    MdcEcgLeadMCL,
    MdcEcgLeadMCL1,
    MdcEcgLeadMCL2,
    MdcEcgLeadMCL3,
    MdcEcgLeadMCL4,
    MdcEcgLeadMCL5,
    MdcEcgLeadMCL6,
    MdcEcgLeadCC,
    MdcEcgLeadCC1,
    MdcEcgLeadCC2,
    MdcEcgLeadCC3,
    MdcEcgLeadCC4,
    MdcEcgLeadCC6,
    MdcEcgLeadCC7,
    MdcEcgLeadCM,
    MdcEcgLeadCM1,
    MdcEcgLeadCM2,
    MdcEcgLeadCM3,
    MdcEcgLeadCM4,
    MdcEcgLeadCM6,
    MdcEcgLeadDIII,
    MdcEcgLeadDAVR,
    MdcEcgLeadDAVL,
    MdcEcgLeadDAVF,
    // Gap: 116..=120 not assigned
    MdcEcgLeadCM7 = 121,
    MdcEcgLeadCH5,
    MdcEcgLeadCS5,
    MdcEcgLeadCB5,
    MdcEcgLeadCR5,
    MdcEcgLeadML,
    MdcEcgLeadAB1,
    MdcEcgLeadAB2,
    MdcEcgLeadAB3,
    MdcEcgLeadAB4,
    MdcEcgLeadES,
    MdcEcgLeadAS,
    MdcEcgLeadAI,
    MdcEcgLeadS,
    // Gap: 135..=146 not assigned
    MdcEcgLeadRL = 147,
    MdcEcgLeadCV5RL,
    MdcEcgLeadCV6LL,
    MdcEcgLeadCV6LU,
    MdcEcgLeadV10,
}
