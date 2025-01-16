use std::convert::Into;

#[derive(Debug)]
pub struct GnssAvailableSatellites {
    gp: u8,
    gl: u8,
    ga: u8,
    gb: u8,
    gi: u8,
    gq: u8,
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

#[derive(Debug)]
pub struct GnssNumericMeasurement {
    is_available: bool,
    value: f64,
    unit_of_measurement: char,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GnssLatLongMeasurement {
    is_available: bool,
    degrees: i32,
    minutes: f32,
    indicator: GnssLatLongIndicator,
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


#[derive(Debug)]
pub struct UtcDateTime {
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: f32,
    is_available: bool,
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