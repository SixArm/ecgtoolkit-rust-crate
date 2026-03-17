# ECG Toolkit Rust Crate - Conversion Plan

## Overview

Convert the C# ECG Conversion Toolkit to Rust. Original author: MJB van Ettinger.
Source: ~/git/sixarm/ecgtoolkit-csharp (149 C# files)

## Architecture

The Rust crate will be a library crate with the following module structure:

```
src/
├── lib.rs                    # Crate root, re-exports
├── types/                    # Core type definitions
│   ├── mod.rs
│   ├── lead_type.rs          # LeadType enum (100+ variants)
│   ├── date.rs               # Date type
│   └── enums.rs              # Sex, Race, AgeDefinition, etc.
├── signal/                   # ECG signal data
│   ├── mod.rs
│   ├── signal.rs             # Single lead/channel
│   ├── signals.rs            # Container for all leads
│   ├── qrs_zone.rs           # QRS zone detection
│   └── buffered_signals.rs   # Memory-efficient streaming
├── demographics/             # Patient demographics
│   ├── mod.rs
│   ├── demographic.rs        # IDemographic trait
│   ├── acquiring_device_id.rs
│   └── drug.rs
├── diagnostic/               # Diagnostic statements
│   ├── mod.rs
│   └── statements.rs
├── measurements/             # ECG measurements
│   ├── mod.rs
│   ├── global_measurements.rs
│   ├── lead_measurements.rs
│   ├── spike.rs
│   └── morphology.rs
├── dsp/                      # Digital signal processing
│   ├── mod.rs
│   ├── filter.rs             # IFilter trait
│   ├── bandpass.rs           # Butterworth bandpass
│   ├── lowpass.rs            # Butterworth lowpass
│   ├── highpass.rs           # Butterworth highpass
│   ├── fir.rs                # FIR filter
│   └── iir.rs                # IIR filter
├── converter/                # Core converter
│   ├── mod.rs
│   ├── ecg_format.rs         # IECGFormat trait
│   ├── ecg_converter.rs      # Main converter
│   ├── ecg_plugin.rs         # Plugin descriptor
│   ├── ecg_config.rs         # Configuration
│   └── ecg_reader.rs         # Reader trait
├── tools/                    # Utility functions
│   ├── mod.rs
│   ├── ecg_tool.rs           # Lead calculations, resampling
│   ├── bytes_tool.rs         # Byte manipulation
│   ├── crc_tool.rs           # CRC calculations
│   └── sorted_list.rs        # Sorted list
├── formats/                  # Format implementations
│   ├── mod.rs
│   ├── scp/                  # SCP format (12 files)
│   ├── aecg/                 # HL7 aECG XML (42 files)
│   ├── dicom/                # DICOM format
│   ├── ishne/                # ISHNE format
│   ├── musexml/              # MUSE XML format
│   ├── omron/                # Omron ECG format
│   ├── csv/                  # CSV export
│   ├── pdf/                  # PDF export
│   └── raw/                  # Raw binary format
└── error.rs                  # Error types
```

## Phases

### Phase 1: Core Types & Enums
- LeadType enum
- Date type
- Demographic enums (Sex, Race, AgeDefinition, HeightDefinition, WeightDefinition)
- Error types

### Phase 2: Signal Types
- Signal struct
- Signals container
- QRSZone
- BufferedSignals

### Phase 3: Demographics & Diagnostics
- IDemographic trait + types
- AcquiringDeviceID
- Drug
- IDiagnostic trait + Statements

### Phase 4: Measurements
- GlobalMeasurements
- LeadMeasurements
- Spike, Morphology, MeasurementType

### Phase 5: DSP Filters
- IFilter trait
- Butterworth bandpass/lowpass/highpass
- FIR and IIR filters

### Phase 6: Converter Core
- IECGFormat trait
- ECGConverter
- ECGPlugin, ECGConfig
- ECGReader trait
- ECGTool utilities

### Phase 7: Format Implementations
- SCP format
- aECG format
- DICOM format
- ISHNE format
- MUSE XML format
- Omron format
- CSV format
- PDF format
- Raw format

### Phase 8: CLI Tool
- Command-line interface matching ECGTool functionality

## Rust Conventions

- Use `#[derive(Clone, Debug, Default)]` where appropriate
- Use `thiserror` for error types
- Use traits instead of C# interfaces/abstract classes
- Use enums instead of C# enums (with repr attributes where needed)
- Use `std::io::Read/Write` for I/O
- Avoid unsafe code where possible
- Use `f64` for signal data (matching C# double)
