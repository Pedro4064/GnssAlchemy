use std::{fs::File, io::Read};
use crate::evkm10::M10GnssDataPoint;

pub mod evkm10;

fn main() {
    println!("Reading GNSS File...");
    let mut file = File::open("../data/data_dump.gnss").expect("Unable to open file");
    let mut buffer = [0;120] ;
    let N = file.read(&mut buffer);

    let data_point= M10GnssDataPoint::from_bytes(&buffer);
    println!("{:?}", data_point);
}
