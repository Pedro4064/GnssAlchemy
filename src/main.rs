use crate::evkm10::M10GnssDataPoint;
use csv::WriterBuilder;
use std::{fs::File, io::Read};

pub mod evkm10;

fn main() {
    println!("Reading GNSS File...");
    let mut file = File::open("../data/data_dump.gnss").expect("Unable to open file");
    let mut buffer = [0; 120];
    let _ = file.read(&mut buffer);

    let data_point = M10GnssDataPoint::from_bytes(&buffer);
    println!("{:?}", data_point);

    let mut csv_writer = WriterBuilder::new()
        .has_headers(false)
        .from_path("potato.csv")
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

    // csv_writer.serialize(data_point).unwrap();
    csv_writer
        .write_record(data_point.serialize_to_string_vec())
        .expect("Unable to write data point to csv file");
}
