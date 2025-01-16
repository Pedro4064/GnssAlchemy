use crate::evkm10::gnss_data::{
    GnssAvailableSatellites, GnssLatLongMeasurement, GnssNumericMeasurement, UtcDateTime,
};

pub mod gnss_data;

#[derive(Debug)]
pub struct M10GnssDataPoint {
    available_satellites: GnssAvailableSatellites,
    latitute: GnssLatLongMeasurement,
    longitude: GnssLatLongMeasurement,
    course_over_ground: GnssNumericMeasurement,
    speed_over_ground: GnssNumericMeasurement,
    time_of_sample: UtcDateTime,
}

pub struct M10GnssDataSet {
    data_points: Vec<M10GnssDataPoint>,
    bytes_per_element: u128,
}

impl M10GnssDataPoint {
    pub fn from_bytes(bytes: &[u8; 120]) -> Self {
        M10GnssDataPoint {
            available_satellites: GnssAvailableSatellites::from_bytes(
                bytes[..6]
                    .try_into()
                    .expect("Unable to parse bytes from available satellites"),
            ),
            // Skip 2 Padding Bytes
            latitute: GnssLatLongMeasurement::from_bytes(
                bytes[8..8 + 16]
                    .try_into()
                    .expect("Unable to parse bytes from latitude"),
            ),
            longitude: GnssLatLongMeasurement::from_bytes(
                bytes[24..24 + 16]
                    .try_into()
                    .expect("Unable to parse bytes from longitude"),
            ),
            course_over_ground: GnssNumericMeasurement::from_bytes(
                bytes[40..40 + 24]
                    .try_into()
                    .expect("Unable to parse bytes for Course Over Ground"),
            ),
            speed_over_ground: GnssNumericMeasurement::from_bytes(
                bytes[64..64 + 24]
                    .try_into()
                    .expect("Unable to parse bytes for Course Over Ground"),
            ),
            time_of_sample: UtcDateTime::from_bytes(
                bytes[88..88 + 16]
                    .try_into()
                    .expect("Unable to parse bytes for time of sample"),
            ),
        }
    }
}

impl M10GnssDataSet {
    pub fn from_bin_dump() -> Self {
        todo!();
    }

    pub fn to_csv(&self) {}
}
