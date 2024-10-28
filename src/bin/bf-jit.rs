use brainforge::instruction::IntermediateInstruction;
use brainforge::jit::JitProgram;
use brainforge::optimizer::{optimize, OptimizerOptions};
use brainforge::{input, BFResult};

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

    /// The size of the memory tape
    #[arg(short, long, default_value_t = 8192)]
    memsize: usize,

    /// Whether to perform simple loop flattening
    #[arg(short, long)]
    loops: bool,

    /// Whether to perform memory scan vectorization
    #[arg(short, long)]
    scan: bool,
}

/// Main program entry point
fn main() -> BFResult<()> {
    let args = CliArgs::parse();

    let src = input(args.file)?;

    let instrs = IntermediateInstruction::parse_instrs(&src)?;
    let optimizer_opts = OptimizerOptions::new()
        .coalesce(true)
        .simple_loops(args.loops)
        .scans(args.scan);
    let optimized_instrs = optimize(instrs, optimizer_opts);

    let memory: Vec<u8> = vec![0; args.memsize];
    let memory_center =
        unsafe { memory.as_ptr().offset((args.memsize / 2) as isize) as *mut libc::c_void };

    // Should be plenty of room
    let num_pages = 100_000;
    let mut program = JitProgram::new(&*optimized_instrs, num_pages)?;
    program.run(memory_center)
}
