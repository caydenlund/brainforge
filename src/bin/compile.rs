//! An executable that compiles the given BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use brainforge::{generator::*, instruction::Instruction, BFError, BFResult};
use clap::Parser;
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

/// The command-line arguments used
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// The file to run
    ///
    /// If one is not provided, then reads a program from stdin
    file: Option<PathBuf>,

    /// The output file
    #[arg(short, long, default_value = "a.s")]
    output: PathBuf,

    /// The size of the memory tape
    #[arg(short, long, default_value_t = 4096)]
    memsize: usize,
}

/// Main program entry point.
fn main() -> BFResult<()> {
    let args = CliArgs::parse();

    let mut src: Vec<u8> = vec![];

    if let Some(filename) = args.file {
        // Read program from file.
        if let Ok(mut file) = File::open(&filename) {
            let Ok(_) = file.read_to_end(&mut src) else {
                return Err(BFError::FileReadError(filename));
            };
        } else {
            return Err(BFError::FileReadError(filename));
        };
    } else {
        // Read program from stdin.
        src = stdin()
            .bytes()
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .collect();
    };

    let instrs = Instruction::parse_instrs(&src)?;

    let mut output: Box<dyn Write> = {
        if args.output == PathBuf::from("-") {
            Box::new(stdout())
        } else {
            let Ok(file) = std::fs::File::create(&args.output) else {
                return Err(BFError::FileWriteError(args.output));
            };
            Box::new(file)
        }
    };

    match output.write(generate(&instrs, args.memsize, Architecture::AMD64).as_bytes()) {
        Err(_) => return Err(BFError::FileWriteError(args.output)),
        _ => {}
    }

    Ok(())
}
