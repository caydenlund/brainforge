//! An executable that interprets the given BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use brainforge::{input, instruction::{BasicInstruction, IntermediateInstruction}, interpreter::*, optimizer::{optimize, OptimizerOptions}, BFResult};
use clap::Parser;
use std::path::PathBuf;

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
    #[arg(short, long, default_value_t = 8192)]
    memsize: usize,

    /// Whether to perform simple loop flattening
    #[arg(short, long)]
    loops: bool,
}

/// Main program entry point.
fn main() -> BFResult<()> {
    let args = CliArgs::parse();

    let src = input(args.file)?;

    let instrs = IntermediateInstruction::parse_instrs(&src)?;
    let optimizer_opts = OptimizerOptions::new()
        .coalesce(true)
        .simple_loops(args.loops);
    let optimized_instrs = optimize(instrs, optimizer_opts);

    interp2(&optimized_instrs, args.memsize);

    // let instrs = BasicInstruction::parse_instrs(&src)?;

    // interpret(&instrs, args.memsize);
    // if args.profile {
    //     interpret_profile(&instrs, args.memsize);
    // } else {
    //     interpret(&instrs, args.memsize);
    // }

    Ok(())
}
