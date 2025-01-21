use clap::Parser;
use std::path::PathBuf;

use evkm10::M10GnssDataSet;
pub mod evkm10;

#[derive(Parser)]
struct CliArguments {
    bin_file_path: PathBuf,
    csv_file_path: PathBuf,
}

fn main() {

    let args: CliArguments = CliArguments::parse();

    let data_set = M10GnssDataSet::from_bin_dump(args.bin_file_path);
    data_set.to_csv(args.csv_file_path);
}
