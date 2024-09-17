//! An executable that interprets the given BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use brainforge::{instruction::Instruction, interpreter::*, BFError, BFResult};
use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

/// The command-line arguments used
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// The file to run
    ///
    /// If one is not provided, then reads a program from stdin
    file: Option<PathBuf>,

    /// Whether to profile the given program
    #[arg(short, long, default_value_t = false)]
    profile: bool,

    /// The size of the memory tape
    #[arg(short, long, default_value_t = 4096)]
    memsize: usize,
}

fn main() -> BFResult<()> {
    let args = CliArgs::parse();

    let mut src: Vec<u8> = vec![];

    if let Some(filename) = args.file {
        if let Ok(mut file) = File::open(&filename) {
            let Ok(_) = file.read_to_end(&mut src) else {
                return Err(BFError::FileReadError(filename));
            };
        } else {
            return Err(BFError::FileReadError(filename));
        };
    };

    let instrs: Vec<Instruction> = Instruction::parse_instrs(&src)?;

    interpret(&instrs, args.memsize);

    Ok(())
}
