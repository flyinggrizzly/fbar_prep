use clap::Parser;

#[derive(Parser)]
struct Args {
    // Path to the FBAR statement data to parse and generate reports for
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Generating FBAR data from {:?}...", args.path);
}
