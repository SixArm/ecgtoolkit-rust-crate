// QRS zone
//
// Class containing a QRS zone.
// Original author: Maarten JB van Ettinger.

/// A QRS zone with type, start, fiducial point, and end.
#[derive(Clone, Debug, Default)]
pub struct QrsZone {
    pub zone_type: u16,
    pub start: i32,
    pub fiducial: i32,
    pub end: i32,
}

impl QrsZone {
    pub fn new(zone_type: u16, start: i32, fiducial: i32, end: i32) -> Self {
        Self { zone_type, start, fiducial, end }
    }
}
