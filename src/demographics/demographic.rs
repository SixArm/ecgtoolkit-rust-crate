// Demographic trait
//
// Interface for manipulation of demographics information.
// Original author: Maarten JB van Ettinger.

use crate::demographics::{AcquiringDeviceId, Drug};
use crate::types::{AgeDefinition, Date, HeightDefinition, Race, Sex, WeightDefinition};

/// Interface for manipulation of demographics information.
pub trait Demographic {
    /// Initialize demographics information.
    fn init(&mut self);

    fn last_name(&self) -> Option<&str>;
    fn set_last_name(&mut self, name: Option<&str>);

    fn first_name(&self) -> Option<&str>;
    fn set_first_name(&mut self, name: Option<&str>);

    fn patient_id(&self) -> Option<&str>;
    fn set_patient_id(&mut self, id: Option<&str>);

    fn second_last_name(&self) -> Option<&str>;
    fn set_second_last_name(&mut self, name: Option<&str>);

    fn prefix_name(&self) -> Option<&str>;
    fn set_prefix_name(&mut self, name: Option<&str>);

    fn suffix_name(&self) -> Option<&str>;
    fn set_suffix_name(&mut self, name: Option<&str>);

    /// Get the age of the patient.
    fn patient_age(&self) -> Option<(u16, AgeDefinition)>;
    /// Set the age of the patient.
    fn set_patient_age(&mut self, val: u16, def: AgeDefinition) -> i32;

    fn patient_birth_date(&self) -> Option<&Date>;
    fn set_patient_birth_date(&mut self, date: Option<Date>);

    /// Get the height of the patient.
    fn patient_height(&self) -> Option<(u16, HeightDefinition)>;
    /// Set the height of the patient.
    fn set_patient_height(&mut self, val: u16, def: HeightDefinition) -> i32;

    /// Get the weight of the patient.
    fn patient_weight(&self) -> Option<(u16, WeightDefinition)>;
    /// Set the weight of the patient.
    fn set_patient_weight(&mut self, val: u16, def: WeightDefinition) -> i32;

    fn gender(&self) -> Sex;
    fn set_gender(&mut self, sex: Sex);

    fn patient_race(&self) -> Race;
    fn set_patient_race(&mut self, race: Race);

    fn acq_machine_id(&self) -> Option<&AcquiringDeviceId>;
    fn set_acq_machine_id(&mut self, id: Option<AcquiringDeviceId>);

    fn analyzing_machine_id(&self) -> Option<&AcquiringDeviceId>;
    fn set_analyzing_machine_id(&mut self, id: Option<AcquiringDeviceId>);

    fn time_acquisition(&self) -> Option<u64>;
    fn set_time_acquisition(&mut self, time: Option<u64>);

    fn baseline_filter(&self) -> u16;
    fn set_baseline_filter(&mut self, val: u16);

    fn lowpass_filter(&self) -> u16;
    fn set_lowpass_filter(&mut self, val: u16);

    fn filter_bitmap(&self) -> u8;
    fn set_filter_bitmap(&mut self, val: u8);

    fn free_text_fields(&self) -> Option<&[String]>;
    fn set_free_text_fields(&mut self, fields: Option<Vec<String>>);

    fn sequence_nr(&self) -> Option<&str>;
    fn set_sequence_nr(&mut self, nr: Option<&str>);

    fn acq_institution(&self) -> Option<&str>;
    fn set_acq_institution(&mut self, name: Option<&str>);

    fn analyzing_institution(&self) -> Option<&str>;
    fn set_analyzing_institution(&mut self, name: Option<&str>);

    fn acq_department(&self) -> Option<&str>;
    fn set_acq_department(&mut self, name: Option<&str>);

    fn analyzing_department(&self) -> Option<&str>;
    fn set_analyzing_department(&mut self, name: Option<&str>);

    fn referring_physician(&self) -> Option<&str>;
    fn set_referring_physician(&mut self, name: Option<&str>);

    fn overreading_physician(&self) -> Option<&str>;
    fn set_overreading_physician(&mut self, name: Option<&str>);

    fn technician_description(&self) -> Option<&str>;
    fn set_technician_description(&mut self, desc: Option<&str>);

    fn systolic_blood_pressure(&self) -> u16;
    fn set_systolic_blood_pressure(&mut self, val: u16);

    fn diastolic_blood_pressure(&self) -> u16;
    fn set_diastolic_blood_pressure(&mut self, val: u16);

    fn drugs(&self) -> Option<&[Drug]>;
    fn set_drugs(&mut self, drugs: Option<Vec<Drug>>);

    fn referral_indication(&self) -> Option<&[String]>;
    fn set_referral_indication(&mut self, indication: Option<Vec<String>>);

    fn room_description(&self) -> Option<&str>;
    fn set_room_description(&mut self, desc: Option<&str>);

    fn stat_code(&self) -> u8;
    fn set_stat_code(&mut self, code: u8);
}
