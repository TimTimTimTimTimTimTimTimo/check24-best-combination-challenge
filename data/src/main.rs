use std::path::PathBuf;

use best_combination_data::Data;
use clap::Parser;
use fehler::throws;

const DEFAULT_GAME_PATH: &str = "./bc_game.csv";
const DEFAULT_OFFER_PATH: &str = "./bc_streaming_offer.csv";
const DEFAULT_PACKAGE_PATH: &str = "./bc_streaming_package.csv";

const DEFAULT_TS_TYPE_PATH: &str = "../src/lib/generated_types.ts";
const DEFAULT_BIN_DATA_PATH: &str = "../src-tauri/betting_game.dat";

#[derive(clap::Parser, Debug)]
struct Config {
    #[arg(default_value = DEFAULT_GAME_PATH)]
    games_path: PathBuf,
    #[arg(default_value = DEFAULT_OFFER_PATH)]
    offers_path: PathBuf,
    #[arg(default_value = DEFAULT_PACKAGE_PATH)]
    packages_path: PathBuf,
    #[arg(default_value = DEFAULT_TS_TYPE_PATH)]
    output_ts_types_path: PathBuf,
    #[arg(default_value = DEFAULT_BIN_DATA_PATH)]
    output_bin_data_path: PathBuf,
}

#[throws(anyhow::Error)]
fn main() {
    let config = Config::parse();

    let data = Data::load_from_csv(
        &config.games_path,
        &config.offers_path,
        &config.packages_path,
    )?;
    data.write_to_bin_file(&config.output_bin_data_path)?;
    data.write_ts_types(&config.output_ts_types_path)?;
}
