//! CLI binary for the DTS bundler.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    typack::cli::run_cli(&args);
}
