use crate::evkm10::gnss_data::{
    GnssAvailableSatellites, GnssLatLongMeasurement, GnssNumericMeasurement, UtcDateTime,
};
use csv::WriterBuilder;
use serde::Serialize;
use std::{fs::File, io::Read, path::PathBuf};

pub mod gnss_data;

#[derive(Debug, Serialize)]
pub struct M10GnssDataPoint {
    available_satellites: GnssAvailableSatellites,
    latitude: GnssLatLongMeasurement,
    longitude: GnssLatLongMeasurement,
    course_over_ground: GnssNumericMeasurement,
    speed_over_ground: GnssNumericMeasurement,
    time_of_sample: UtcDateTime,
}

impl M10GnssDataPoint {
    /// Deserialize gnss data-point from binary data.
    ///
    /// # Arguments
    ///
    /// - `bytes` - Reference to a slice of type u8 (byte)
    ///
    /// # Returns
    ///
    /// Instance of `M10GnssDataPoint` from binary file.
    pub fn from_bytes(bytes: &[u8]) -> Self {
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

    /// Converts every field in the struct into a vec of their corresponding strings version,
    /// as to use the csv crate to write the record to file.
    ///
    /// # Returns
    ///
    /// Vector containing the string version of all struct fields.
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

pub struct M10GnssDataSet {
    data_points: Vec<M10GnssDataPoint>,
}

impl M10GnssDataSet {
    /// Deserialize data-points from binary file
    ///
    /// # Arguments
    ///
    /// - `dump_file_path` - Path to .gnss binary file
    ///
    /// # Returns
    ///
    /// An instance of `M10GnssDataSet`, containing all data points parsed.
    /// If the last datapoint was not entirely saved to file, it will ignore the
    /// last impartial datapoint.
    pub fn from_bin_dump(dump_file_path: PathBuf) -> Self {
        let mut file = File::open(dump_file_path).expect("Unable to open file");
        let mut bin_content: Vec<u8> = Vec::new();

        file.read_to_end(&mut bin_content)
            .expect("Unable to read bin data dump");
        M10GnssDataSet {
            data_points: bin_content
                .chunks_exact(120)
                .map(|bin_chunk| M10GnssDataPoint::from_bytes(bin_chunk))
                .collect(),
        }
    }

    /// Export data points to a csv file with the following headers:
    ///
    /// # Arguments
    ///
    /// - `csv_file_path` - Path to csv file to be written
    pub fn to_csv(&self, csv_file_path: PathBuf) {
        let mut csv_writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(csv_file_path)
            .expect("Unable to create/open csv file");

        csv_writer
            .write_record([
                "gp",
                "gl",
                "ga",
                "gb",
                "gi",
                "gq",
                "Latitude: Available",
                "Latitude: Degrees",
                "Latitude: Minutes",
                "Latitude: Indicator",
                "Longitude: Available",
                "Longitude: Degrees",
                "Longitude: Minutes",
                "Longitude: Indicator",
                "Course Over Ground: Available",
                "Course Over Ground: Value",
                "Course Over Ground: Unit Of Measurement",
                "Speed Over Ground: Available",
                "Speed Over Ground: Value",
                "Speed Over Ground: Unit Of Measurement",
                "Timestamp: Year",
                "Timestamp: Month",
                "Timestamp: Day",
                "Timestamp: Hour",
                "Timestamp: Minute",
                "Timestamp: Second",
                "Timestamp: Available",
            ])
            .expect("Unable to Write Headers to .csv file");

        for data_point in &self.data_points {
            csv_writer
                .write_record(data_point.serialize_to_string_vec())
                .expect("Unable to write data point to csv file");
        }
    }
}
