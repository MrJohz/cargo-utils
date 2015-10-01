extern crate clap;

use std::env;

fn main() {
    let matches = clap::App::new("cargo-coverage")
        .version("0.0.1")
        .about("Collect coverage data for rust projects")
        .arg(clap::Arg::with_name("report-directory")
            .short("r")
            .long("report-directory")
            .takes_value(true)
            .help("Sets the directory to save the report to"))
        .arg(clap::Arg::with_name("cache-directory")
            .long("cache-directory")
            .takes_value(true)
            .help("Sets the directory to store vcov binaries in"))
        .get_matches();

    let current_dir = env::current_dir().unwrap_or_else(|err|
        panic!("Error finding working directory: {:?}", err));
    let home_dir = env::home_dir().unwrap_or_else(||
        panic!("No home directory found"));

    let coverage_directory = current_dir.join(matches
        .value_of("report-directory")
        .unwrap_or("target/coverage"));

    let cache_directory = home_dir.join(matches
        .value_of("cache-directory")
        .unwrap_or(".cache/cargo-coverage/vcov"));
}
