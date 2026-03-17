# ECG Toolkit Rust Crate - Documentation

## Overview

The ECG Toolkit is a Rust library for reading, writing, converting, and
processing electrocardiogram (ECG) medical data files. It is a conversion of the
C# ECG Conversion Toolkit originally written by Maarten JB van Ettinger.

- **License**: Apache License, Version 2.0
- **Original author**: Maarten JB van Ettinger
- **Original source**: van Ettinger Information Technology / Thoraxcentrum, Erasmus MC

## Crate Structure

The crate is organized into seven modules:

```
ecgtoolkit/
├── types/          Core type definitions (LeadType, Date, enums)
├── signal/         ECG signal data (Signal, Signals, QrsZone)
├── demographics/   Patient demographic data (Demographic trait, Drug, AcquiringDeviceId)
├── diagnostic/     Diagnostic statements (Statements, DiagnosticStatements trait)
├── measurements/   ECG measurements (GlobalMeasurement, LeadMeasurement, Spike, Morphology)
├── dsp/            Digital signal processing (Butterworth filters, FIR, IIR)
├── converter/      Format conversion interfaces (EcgFormat trait, EcgConfig)
└── tools/          Utility functions (lead calculations, resampling, anonymization)
```

---

## Module: `types`

Core type definitions used throughout the library.

### `LeadType`

Enumeration of 150+ ECG lead types. Matches the C# `LeadType` enum values exactly.

```rust
use ecgtoolkit::types::LeadType;

let lead = LeadType::V1;
let lead_from_byte = LeadType::from_u8(3); // V1
assert_eq!(lead, lead_from_byte);
```

Standard 12-lead types: `I`, `II`, `III`, `AVR`, `AVL`, `AVF`, `V1`-`V6`.

Extended types include: `V7`-`V9`, `V2R`-`V7R`, `X`, `Y`, `Z`, Frank leads,
derived leads (prefixed with `D`), and many more.

The `from_u8` method safely converts a byte to a `LeadType`, returning
`Unknown` for out-of-range values.

### `LeadTypeVitalRefId`

MDC (Medical Device Communication) reference IDs for lead types. Maps to
standard vital signs identifiers (e.g., `MdcEcgLeadI = 1`, `MdcEcgLeadIII = 61`).

### `Date`

Simple date type with year, month, and day fields, matching the SCP format.

```rust
use ecgtoolkit::types::Date;

let date = Date::new(2024, 6, 15);
assert!(date.is_existing_date());

// Leap year validation
assert!(Date::new(2024, 2, 29).is_existing_date());
assert!(!Date::new(2023, 2, 29).is_existing_date());
```

### Demographic Enums

| Enum | Variants |
|------|----------|
| `Sex` | Unspecified, Male, Female, Null |
| `Race` | Unspecified, Caucasian, Black, Oriental, Null |
| `AgeDefinition` | Unspecified, Years, Months, Weeks, Days, Hours |
| `HeightDefinition` | Unspecified, Centimeters, Inches, Millimeters |
| `WeightDefinition` | Unspecified, Kilogram, Gram, Pound, Ounce |
| `DeviceType` | Cart, System |
| `DeviceManufacturer` | Unknown, Burdick, Cambridge, ... Other (21 variants) |
| `ElectrodeConfigCodeTwelveLead` | Unspecified, StandardTwelveLead, ... (7 variants) |
| `ElectrodeConfigCodeXyz` | Unspecified, Frank, ... (7 variants) |

---

## Module: `signal`

ECG signal data structures.

### `Signal`

Represents one ECG lead/channel.

```rust
use ecgtoolkit::signal::Signal;
use ecgtoolkit::types::LeadType;

let sig = Signal {
    lead_type: LeadType::I,
    rhythm_start: 0,
    rhythm_end: 1000,
    rhythm: Some(vec![0i16; 1000]),
    median: None,
};
```

**Key methods:**

- `apply_filter(rhythm_filter, median_filter)` — Apply DSP filters, returning a new Signal
- `Signal::is_normal(leads)` — Check if first 8 leads are I, II, V1-V6
- `Signal::nr_simultaneously(leads)` — Count leads recorded at the same time (within 5-sample tolerance)
- `Signal::sort_on_type(leads)` — Sort leads by type (quicksort)

### `Signals`

Container for all ECG signal leads with metadata.

```rust
use ecgtoolkit::signal::Signals;

let mut sigs = Signals::with_nr_leads(12);
sigs.rhythm_avm = 2.5;              // Analog value multiplier in mV
sigs.rhythm_samples_per_second = 500;
```

**Key methods:**

| Method | Description |
|--------|-------------|
| `nr_leads()` / `set_nr_leads()` | Get/set number of leads |
| `get(i)` / `get_mut(i)` | Access lead by index |
| `set_leads(leads)` | Set all leads (max 255) |
| `is_normal()` | Check standard 8-lead order |
| `is_twelve_leads()` | Detect 12-lead configuration |
| `is_fifteen_leads()` | Detect 15-lead configuration (both variants) |
| `calculate_twelve_leads()` | Derive 12 leads from 8 (I, II, V1-V6) |
| `calculate_start_and_end()` | Get signal time boundaries |
| `nr_simultaneously()` | Count simultaneous leads |
| `sort_on_type()` | Sort leads by type |
| `trim_signals(val)` | Remove leading/trailing padding |
| `resample(sps)` | Resample all leads to new rate |
| `set_avm(avm)` | Change analog value multiplier |
| `apply_bandpass_filter(lo, hi)` | Apply Butterworth bandpass filter |
| `apply_lowpass_filter(cutoff)` | Apply Butterworth lowpass filter |
| `apply_highpass_filter(cutoff)` | Apply Butterworth highpass filter |

### `QrsZone`

Describes a QRS complex zone with start, fiducial point, and end positions.

---

## Module: `demographics`

Patient and device identification data.

### `Demographic` Trait

Defines 40+ getter/setter methods for patient demographics. Implementations
provide access to:

- **Patient identity**: last name, first name, patient ID, prefix/suffix
- **Physical data**: age (with units), height, weight, sex, race
- **Device IDs**: acquiring and analyzing machine identifiers
- **Acquisition info**: timestamp, filters, blood pressure
- **Institutional**: institution, department, physicians, technician
- **Clinical**: drugs, referral indication, room description, stat code

### `AcquiringDeviceId`

Device identification with institution number, department, device ID,
manufacturer, capabilities, and model description.

```rust
use ecgtoolkit::demographics::AcquiringDeviceId;

let device = AcquiringDeviceId::no_device();
assert_eq!(&device.model_description[..5], b"MCONV");
```

### `Drug`

Simple drug record with class, code, and text description.

---

## Module: `diagnostic`

### `Statements`

Diagnostic interpretation statements for an ECG.

```rust
use ecgtoolkit::diagnostic::Statements;

let stmt = Statements {
    confirmed: true,
    interpreted: true,
    time: 1700000000,
    statement: Some(vec!["Normal sinus rhythm".to_string()]),
};
```

### `DiagnosticStatements` Trait

Interface for getting and setting diagnostic statements.

---

## Module: `measurements`

ECG measurement data at both global and per-lead levels.

### `GlobalMeasurement`

One wave measurement (SCP/UNIPRO defined) containing P/QRS/T onset/offset
values and axis measurements.

```rust
use ecgtoolkit::measurements::GlobalMeasurement;

let mut m = GlobalMeasurement::default();
m.set_p_dur(80);      // P-wave duration: 80ms
m.set_qrs_dur(100);   // QRS duration: 100ms
m.set_qt_dur(400);    // QT duration: 400ms

// Calculate corrected QT (Bazett formula)
use ecgtoolkit::measurements::QTcCalcType;
let qtc = m.calc_qtc(800, 75, QTcCalcType::Bazett); // ~447ms
```

**Sentinel values:**
- `NO_VALUE = 29999` — indicates no measurement available (u16)
- `NO_AXIS_VALUE = 29999` — indicates no axis measurement (i16)

**QTc calculation types:** Bazett, Hodges, Fridericia, Framingham.

### `GlobalMeasurements`

Collection of global measurements including ventricular rate, average RR/PP
intervals, spike data, and QTc configuration.

### `LeadMeasurement`

Per-lead measurement storage using a BTreeMap keyed by `MeasurementType`.

```rust
use ecgtoolkit::measurements::{LeadMeasurement, MeasurementType};
use ecgtoolkit::types::LeadType;

let mut lm = LeadMeasurement::with_lead_type(LeadType::V1);
lm.set(MeasurementType::Ramp, 1500);
lm.set(MeasurementType::Samp, -500);
assert_eq!(lm.count(), 2);
```

### `MeasurementType`

44 measurement types including durations (Pdur, QRSdur, QTint), amplitudes
(Qamp, Ramp, Samp), ST measurements, morphology indices, and wave offsets.

Aliases: `RONSET = Qoffset`, `SONSET = Roffset`, etc.

### `Morphology`

Wave morphology classification: Positive, Negative, PositiveNegative,
NegativePositive, NotchedMShaped, NotchedWShaped, etc.

### `Spike`

Pacemaker spike data with time and amplitude.

---

## Module: `dsp`

Digital signal processing filters for ECG data.

### `Filter` Trait

```rust
pub trait Filter {
    fn compute(&mut self, input: f64) -> f64;
}
```

All filters implement this trait, allowing runtime polymorphism via
`Box<dyn Filter>`.

### Butterworth Filters

Multi-section Butterworth filter implementations for ECG signal processing:

| Filter | Usage |
|--------|-------|
| `LowpassFilterButterworth` | Remove high-frequency noise |
| `HighpassFilterButterworth` | Remove baseline wander |
| `BandpassFilterButterworth` | Isolate frequency band (cascade of lowpass + highpass) |

```rust
use ecgtoolkit::dsp::{Filter, BandpassFilterButterworth};

let mut filter = BandpassFilterButterworth::new(
    0.5,    // bottom frequency (Hz)
    40.0,   // top frequency (Hz)
    2,      // number of sections
    500.0,  // sample rate (Hz)
);

let output = filter.compute(input_sample);
```

Each filter is composed of sections (`LowpassFilterButterworthSection`,
`HighpassFilterButterworthSection`), which internally use FIR and IIR
filter building blocks.

### `FirFilter` / `IirFilter`

Low-level Finite Impulse Response and Infinite Impulse Response filter
implementations used by the Butterworth filter sections.

---

## Module: `converter`

Core interfaces for ECG file format conversion.

### `EcgFormat` Trait

Abstract interface that all ECG file format implementations must satisfy.
Defines methods for:

- Reading from files, streams, or byte buffers
- Writing to files, streams, or byte buffers
- Format detection (`check_format_*`)
- Access to demographics, diagnostics, signals, and measurements
- Anonymization
- Format validation

### `EcgConfig`

Configuration management for format-specific settings. Supports mandatory
and optional configuration keys with validation.

```rust
use ecgtoolkit::converter::EcgConfig;

let mut cfg = EcgConfig::new(
    &["sample_rate"],           // mandatory
    &["encoding", "compression"], // optional
    None,                        // no custom check function
);

cfg.set("sample_rate", Some("500"));
assert!(cfg.configuration_works());
```

---

## Module: `tools`

Utility functions for ECG signal processing.

### Lead Calculations

Calculate derived ECG leads from primary leads using standard formulas:

| Function | Formula |
|----------|---------|
| `calculate_lead_iii` | III = II - I |
| `calculate_lead_avr` | aVR = -(I + II) / 2 |
| `calculate_lead_avl` | aVL = (2I - II) / 2 or (I - III) / 2 |
| `calculate_lead_avf` | aVF = (2II - I) / 2 or (II + III) / 2 |
| `calculate_leads_from_two` | All four from I and II |
| `calculate_leads_from_three` | aVR, aVL, aVF from I, II, III |

These satisfy Einthoven's law: I + III = II.

### Signal Processing

| Function | Description |
|----------|-------------|
| `lead_subtract(a, b)` | Subtract lead B from A in-place |
| `lead_add(a, b)` | Add lead B to A in-place |
| `change_multiplier(src, old, new)` | Scale signal values |
| `copy_signal(src, dst, ...)` | Copy signal data between buffers |
| `shift_signal(src, shift)` | DC offset removal |

### Resampling

Polynomial interpolation (Neville's algorithm) for sample rate conversion:

```rust
use ecgtoolkit::tools::ecg_tool;

let src = vec![100i16; 500]; // 500 samples at 250 Hz
let dst = ecg_tool::resample_lead(&src, 250, 500); // upsample to 500 Hz
```

### Anonymization

```rust
use ecgtoolkit::tools::ecg_tool;

// Replaces patient names, IDs, and birth date with anonymized values
ecg_tool::anonymous(&mut demographics, '*');
```

---

## ECG Lead Configurations

### Standard 12-Lead ECG

The standard 12-lead ECG consists of:

- **Limb leads**: I, II, III (Einthoven triangle)
- **Augmented leads**: aVR, aVL, aVF (Goldberger)
- **Precordial leads**: V1, V2, V3, V4, V5, V6 (Wilson)

The toolkit can derive the full 12 leads from 8 independent leads (I, II,
V1-V6) using Einthoven's law and augmented lead formulas.

### 15-Lead ECG

Two standard 15-lead configurations are supported:

1. Standard 12 + V3R, V4R, V7 (right-sided and posterior)
2. Standard 12 + V7, V8, V9 (posterior leads)

---

## Testing

The crate includes 184 comprehensive tests across 7 test modules:

```sh
cargo test
```

Tests cover all modules including:
- Type conversions and enum values
- Signal lead detection and sorting
- All four QTc formulas with clinical ranges
- Einthoven's law mathematical verification
- DSP filter frequency response characteristics
- Resampling accuracy
- Configuration management
- 12-lead and 15-lead detection

---

## Future Work

Format implementations (Phase 7) are planned for:

- **SCP** — Standardized Communications Protocol
- **HL7 aECG** — XML-based annotated ECG
- **DICOM** — Medical imaging standard
- **ISHNE** — Holter format
- **MUSE XML** — GE Healthcare format
- **Omron ECG** — Consumer device format
- **CSV** — Comma-separated values export
- **PDF** — PDF rendering
- **Raw** — Raw binary format
