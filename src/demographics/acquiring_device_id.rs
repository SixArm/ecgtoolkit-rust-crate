// Acquiring device ID
//
// Device information for ECG records.
// Original author: Maarten JB van Ettinger.

/// Device information that can be imported and exported in both SCP and UNIPRO.
#[derive(Clone, Debug)]
pub struct AcquiringDeviceId {
    pub institution_nr: u16,
    pub department_nr: u16,
    pub device_id: u16,
    pub device_type: u8,
    pub manufacturer_id: u8,
    /// Defined in SCP Section1 tag 14 byte 18.
    pub device_capabilities: u8,
    /// Defined in SCP Section1 tag 14 byte 19.
    pub ac_frequency_environment: u8,
    pub model_description: [u8; 6],
}

impl Default for AcquiringDeviceId {
    fn default() -> Self {
        Self {
            institution_nr: 0,
            department_nr: 0,
            device_id: 0,
            device_type: 0,
            manufacturer_id: 0, // DeviceManufacturer::Unknown
            device_capabilities: 0,
            ac_frequency_environment: 0,
            model_description: [0u8; 6],
        }
    }
}

impl AcquiringDeviceId {
    /// Create a device ID with no-device defaults.
    pub fn no_device() -> Self {
        let mut id = Self {
            institution_nr: 0,
            department_nr: 11,
            device_id: 51,
            device_type: 1, // DeviceType::System
            manufacturer_id: 0,
            device_capabilities: 0x8,
            ac_frequency_environment: 1,
            model_description: [0u8; 6],
        };
        // Write "MCONV" into model description
        let name = b"MCONV";
        let len = name.len().min(6);
        id.model_description[..len].copy_from_slice(&name[..len]);
        id
    }
}
