use crate::evkm10::gnss_data::{
    GnssAvailableSatellites, GnssLatLongMeasurement, GnssNumericMeasurement, UtcDateTime,
};
use serde::Serialize;

pub mod gnss_data;

#[derive(Debug, Serialize)]
pub struct M10GnssDataPoint {
    #[serde(flatten)]
    available_satellites: GnssAvailableSatellites,

    #[serde(flatten)]
    latitude: GnssLatLongMeasurement,

    #[serde(flatten)]
    longitude: GnssLatLongMeasurement,

    #[serde(flatten)]
    course_over_ground: GnssNumericMeasurement,

    #[serde(flatten)]
    speed_over_ground: GnssNumericMeasurement,

    #[serde(flatten)]
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
            latitude: GnssLatLongMeasurement::from_bytes(
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

    pub fn serialize_to_string_vec(&self) -> Vec<String> {
        vec![
            self.available_satellites.gp.to_string(),
            self.available_satellites.gl.to_string(),
            self.available_satellites.ga.to_string(),
            self.available_satellites.gb.to_string(),
            self.available_satellites.gi.to_string(),
            self.available_satellites.gq.to_string(),
            self.latitude.is_available.to_string(),
            self.latitude.degrees.to_string(),
            self.latitude.minutes.to_string(),
            self.latitude.indicator.to_string(),
            self.longitude.is_available.to_string(),
            self.longitude.degrees.to_string(),
            self.longitude.minutes.to_string(),
            self.longitude.indicator.to_string(),
            self.course_over_ground.is_available.to_string(),
            self.course_over_ground.value.to_string(),
            self.course_over_ground.unit_of_measurement.to_string(),
            self.speed_over_ground.is_available.to_string(),
            self.speed_over_ground.value.to_string(),
            self.speed_over_ground.unit_of_measurement.to_string(),
            self.time_of_sample.year.to_string(),
            self.time_of_sample.month.to_string(),
            self.time_of_sample.day.to_string(),
            self.time_of_sample.hour.to_string(),
            self.time_of_sample.minute.to_string(),
            self.time_of_sample.second.to_string(),
            self.time_of_sample.is_available.to_string(),
        ]
    }
}

impl M10GnssDataSet {
    pub fn from_bin_dump() -> Self {
        todo!();
    }

    pub fn to_csv(&self) {}
}
