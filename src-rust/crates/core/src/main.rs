use std::path::PathBuf;

use argh::FromArgs;
use best_combination_core::data::Data;
use fehler::throws;

const DEFAULT_GAME_PATH: &str = "./data/bc_game.csv";
const DEFAULT_OFFER_PATH: &str = "./data/bc_streaming_offer.csv";
const DEFAULT_PACKAGE_PATH: &str = "./data/bc_streaming_package.csv";

const DEFAULT_TS_TYPE_PATH: &str = "../../../src/lib/generated_types.ts";
const DEFAULT_BIN_DATA_PATH: &str = "./data/best_combination.dat";

#[derive(FromArgs)]
/// Parses the provided CSV Files and then re-exports them as binary data.
/// Also generates TS-types for use in the frontend.
struct Args {
    #[argh(option, default = "PathBuf::from(DEFAULT_GAME_PATH)")]
    /// path to CSV file containing games data.
    games_path: PathBuf,
    #[argh(option, default = "PathBuf::from(DEFAULT_OFFER_PATH)")]
    /// path to CSV file containing offers data.
    offers_path: PathBuf,
    #[argh(option, default = "PathBuf::from(DEFAULT_PACKAGE_PATH)")]
    /// path to CSV file containing packages data.
    packages_path: PathBuf,
    #[argh(option, default = "PathBuf::from(DEFAULT_TS_TYPE_PATH)")]
    /// output path for TS-types.
    output_ts_types_path: PathBuf,
    #[argh(option, default = "PathBuf::from(DEFAULT_BIN_DATA_PATH)")]
    /// output path for binary data.
    output_bin_data_path: PathBuf,
}

#[throws(anyhow::Error)]
fn main() {
    let args: Args = argh::from_env();

    let data = Data::load_from_csv(&args.games_path, &args.offers_path, &args.packages_path)?;
    data.store_as_bin_file(&args.output_bin_data_path)?;
    data.generate_ts_types(&args.output_ts_types_path)?;
}
