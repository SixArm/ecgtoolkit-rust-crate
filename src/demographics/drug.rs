// Drug
//
// Drug information for ECG records.
// Original author: Maarten JB van Ettinger.

/// Drug information (close to SCP and UNIPRO implementations).
#[derive(Clone, Debug, Default)]
pub struct Drug {
    pub drug_class: u8,
    pub class_code: u8,
    pub text_description: Option<String>,
}
