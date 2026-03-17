# ECG Toolkit Rust Crate - Task Tracker

## Completed: Phases 1-6 - Core Library

### Completed Tasks

- [x] Create lib.rs with module structure
- [x] Create types/lead_type.rs - LeadType enum (150+ variants + LeadTypeVitalRefId)
- [x] Create types/date.rs - Date type with validation
- [x] Create types/enums.rs - Sex, Race, AgeDefinition, HeightDefinition, WeightDefinition, DeviceType, DeviceManufacturer, ElectrodeConfigCodes
- [x] Create signal/signal.rs - Signal struct with filter, sort, normalize
- [x] Create signal/signals.rs - Signals container with filter, resample, twelve/fifteen lead support
- [x] Create signal/qrs_zone.rs - QrsZone struct
- [x] Create demographics/demographic.rs - Demographic trait
- [x] Create demographics/acquiring_device_id.rs - AcquiringDeviceId struct
- [x] Create demographics/drug.rs - Drug struct
- [x] Create diagnostic/statements.rs - Statements struct + DiagnosticStatements trait
- [x] Create measurements/global_measurement.rs - GlobalMeasurement with QTc calculation
- [x] Create measurements/global_measurements.rs - GlobalMeasurements container + trait
- [x] Create measurements/lead_measurement.rs - LeadMeasurement with BTreeMap storage
- [x] Create measurements/lead_measurements.rs - LeadMeasurements container + trait
- [x] Create measurements/spike.rs - Spike struct
- [x] Create measurements/morphology.rs - Morphology enum
- [x] Create measurements/measurement_type.rs - MeasurementType enum (44 variants)
- [x] Create dsp/filter.rs - Filter trait
- [x] Create dsp/fir.rs - FIR filter
- [x] Create dsp/iir.rs - IIR filter
- [x] Create dsp/lowpass_section.rs - Butterworth lowpass section
- [x] Create dsp/highpass_section.rs - Butterworth highpass section
- [x] Create dsp/lowpass.rs - Multi-section Butterworth lowpass
- [x] Create dsp/highpass.rs - Multi-section Butterworth highpass
- [x] Create dsp/bandpass.rs - Bandpass (cascade of lowpass + highpass)
- [x] Create converter/ecg_format.rs - EcgFormat trait
- [x] Create converter/ecg_config.rs - EcgConfig struct
- [x] Create tools/ecg_tool.rs - Lead calculations, resampling, multiplier change, anonymization

## Next Phase: Phase 7 - Format Implementations

### Remaining Tasks

- [ ] SCP format (12 files in C#)
- [ ] aECG format (42 files in C#)
- [ ] DICOM format
- [ ] ISHNE format
- [ ] MUSE XML format
- [ ] Omron ECG format
- [ ] CSV format
- [ ] PDF format
- [ ] Raw format
- [ ] CLI tool
