// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fehler::throws;

#[throws(anyhow::Error)]
fn main() {
    best_combination_lib::run()
}
