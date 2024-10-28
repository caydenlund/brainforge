//! An executable that compiles the given BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use brainforge::instruction::IntermediateInstruction;
use brainforge::optimizer::{optimize, OptimizerOptions};
use brainforge::{generator::*, input, output, Architecture, BFError, BFResult};
use clap::Parser;
use std::{io::Write, path::PathBuf};

/// The command-line arguments used
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// The file to run
    ///
    /// If one is not provided, then reads a program from stdin
    file: Option<PathBuf>,

    /// The output file
    ///
    /// Use `-` for stdout
    #[arg(short, long, default_value = "a.s")]
    output: PathBuf,

    /// The size of the memory tape
    #[arg(short, long, default_value_t = 8192)]
    memsize: usize,

    /// Whether to perform simple loop flattening
    #[arg(short, long)]
    loops: bool,

    /// Whether to perform memory scan vectorization
    #[arg(short, long)]
    scan: bool,

    /// Whether to perform partial evaluation
    #[arg(short, long)]
    partial_evaluation: bool,
}

/// Main program entry point.
fn main() -> BFResult<()> {
    let args = CliArgs::parse();

    let src = input(args.file)?;

    let instrs = IntermediateInstruction::parse_instrs(&src)?;
    let optimizer_opts = OptimizerOptions::new()
        .coalesce(true)
        .simple_loops(args.loops)
        .scans(args.scan);
    let optimized_instrs = optimize(instrs, optimizer_opts);

    let mut output = output(&args.output)?;

    match output.write(
        generate(
            &optimized_instrs,
            args.partial_evaluation,
            args.memsize,
            Architecture::AMD64,
        )?
        .as_bytes(),
    ) {
        Err(_) => return Err(BFError::FileWriteError(args.output)),
        _ => {}
    }

    Ok(())
}
