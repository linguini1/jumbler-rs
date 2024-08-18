mod jumbler;

use jumbler::JumblerFSM;
use std::{
    fs::File,
    io::{self, BufReader},
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short)]
    input_file: Option<String>,
    #[arg(short)]
    output_file: Option<String>,
}

fn main() -> Result<(), io::Error> {
    // Parse command line arguments
    let args = Args::parse();

    // Construct FSM
    let mut stdout = io::stdout().lock();
    let mut fsm = JumblerFSM::new(&mut stdout);

    if let Some(output_file) = args.output_file {
        let mut outfile = File::create(output_file)?;
        fsm = JumblerFSM::new(&mut outfile);

        // Run FSM
        if let Some(input_file) = args.input_file {
            fsm.run(BufReader::new(File::open(input_file)?));
        } else {
            fsm.run(io::stdin().lock());
        }
        return Ok(());
    }

    // Run FSM
    if let Some(input_file) = args.input_file {
        fsm.run(BufReader::new(File::open(input_file)?));
    } else {
        fsm.run(io::stdin().lock());
    }

    // Construct and run the FSM
    Ok(())
}
