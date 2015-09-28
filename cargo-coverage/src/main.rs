extern crate clap;

fn main() {
    let matches = clap::App::new("cargo-coverage")
        .version("0.0.1")
        .about("Collect coverage data for rust projects")
        .get_matches();
}
