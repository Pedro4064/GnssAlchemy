use std::{convert::Into, fmt};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GnssAvailableSatellites {
    pub gp: u8,
    pub gl: u8,
    pub ga: u8,
    pub gb: u8,
    pub gi: u8,
    pub gq: u8,
}

impl GnssAvailableSatellites {
    pub fn from_bytes(raw_bytes: [u8; 6]) -> Self {
        GnssAvailableSatellites {
            gp: raw_bytes[0],
            gl: raw_bytes[1],
            ga: raw_bytes[2],
            gb: raw_bytes[3],
            gi: raw_bytes[4],
            gq: raw_bytes[5],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GnssNumericMeasurement {
    pub is_available: bool,
    pub value: f64,
    pub unit_of_measurement: char,
}

impl GnssNumericMeasurement {
    pub fn from_bytes(raw_bytes: [u8; 24]) -> Self {
        GnssNumericMeasurement {
            is_available: raw_bytes[0] != 0,
            value: f64::from_le_bytes(raw_bytes[8..8+8].try_into().expect("Unable to parse f64 value for Gnss Numeric Data")),
            unit_of_measurement: raw_bytes[16].try_into().expect("Unable to parse byte for ASCII of the Engineering Unit for Gnss Numeric Data")
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum GnssLatLongIndicator {
    N,
    S,
    E,
    W,
}

impl Into<GnssLatLongIndicator> for u8 {
    fn into(self) -> GnssLatLongIndicator {
        match self {
            b'N' => GnssLatLongIndicator::N,
            b'S' => GnssLatLongIndicator::S,
            b'E' => GnssLatLongIndicator::E,
            b'W' => GnssLatLongIndicator::W,
            _    => panic!("Error While Parsing GNSS Lattitude and Longitude Indicator - Did not fall in either [N, S, E, W] category")
        }
    }
}

impl fmt::Display for GnssLatLongIndicator {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                GnssLatLongIndicator::N => write!(f, "N"),
                GnssLatLongIndicator::S => write!(f, "S"),
                GnssLatLongIndicator::E => write!(f, "E"),
                GnssLatLongIndicator::W => write!(f, "W"),
            }
        }
    
}

#[derive(Debug, Serialize)]
pub struct GnssLatLongMeasurement {
    pub is_available: bool,
    pub degrees: i32,
    pub minutes: f32,
    pub indicator: GnssLatLongIndicator,
}

impl GnssLatLongMeasurement {
    pub fn from_bytes(raw_bytes: [u8; 16]) -> Self {
        GnssLatLongMeasurement {
            is_available: raw_bytes[0] != 0,

            // Skip 3 Padding Bytes
            degrees: i32::from_le_bytes([raw_bytes[4], raw_bytes[5], raw_bytes[6], raw_bytes[7]]),
            minutes: f32::from_le_bytes([raw_bytes[8], raw_bytes[9], raw_bytes[10], raw_bytes[11]]),
            indicator: raw_bytes[12].into(),
        }
    }
}


#[derive(Debug, Serialize)]
pub struct UtcDateTime {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: f32,
    pub is_available: bool,
}

impl UtcDateTime {
    pub fn from_bytes(raw_bytes: [u8; 16]) -> Self {
        UtcDateTime {
            year: raw_bytes[0],
            month: raw_bytes[1],
            day: raw_bytes[2],
            hour: raw_bytes[3],
            minute: raw_bytes[4],

            // Skip 3 padding bytes
            second: f32::from_le_bytes(raw_bytes[8..8+4].try_into().expect("Unable to parse bytes for UTC seconds as f32")),
            is_available: raw_bytes[12] != 0
        }
    }
}