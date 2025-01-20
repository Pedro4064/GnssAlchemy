use evkm10::M10GnssDataSet;
pub mod evkm10;

fn main() {
    println!("Reading GNSS File...");
    let data_set = M10GnssDataSet::from_bin_dump("../data/data_dump.gnss");
    data_set.to_csv();

}
