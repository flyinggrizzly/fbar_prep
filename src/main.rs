use clap::Parser;

mod data;
mod facts;
mod report_context;

#[derive(Parser)]
struct Args {
    // Path to the FBAR statement data to parse and generate reports for
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Generating FBAR data from {:?}...", args.path);

    // Load facts data
    let facts = match facts::Facts::load_facts() {
        Ok(facts) => {
            println!("Loaded {} years of facts data", facts.years.len());
            facts
        }
        Err(err) => {
            eprintln!("Error loading facts data: {}", err);
            std::process::exit(1);
        }
    };

    let user_data = match data::UserData::load_from_path(&args.path) {
        Ok(data) => {
            println!("Loaded FBAR data: {:?}", data);
            data
        }
        Err(err) => {
            eprintln!("Error loading FBAR data: {}", err);
            std::process::exit(1);
        }
    };

    let context = report_context::ReportContext::new(facts, user_data.fact_extensions);
}
