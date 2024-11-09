//! An executable that uses LLVM to compile the given BF program
//!
//! Author: Cayden Lund (cayden.lund@utah.edu)

use brainforge::assembly::llvm::{LLVMInstruction, LlvmContext};
use brainforge::instruction::IntermediateInstruction;
use brainforge::optimizer::{optimize, OptimizerOptions};
use brainforge::{input, BFError, BFResult};
use clap::Parser;
use inkwell::context::Context;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;
use std::path::PathBuf;

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
    /// Defaults to `bf.o`
    #[arg(short, long, default_value = "bf.o")]
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

    let ctx = Context::create();
    let ctx = LlvmContext::new(&ctx, args.memsize)?;

    Target::initialize_native(&InitializationConfig::default()).map_err(|err| {
        BFError::LlvmError(format!("Failed to initialize native target: `{}`", err))
    })?;
    let target_triple = TargetMachine::get_default_triple();
    ctx.module.set_triple(&target_triple);
    let target = Target::from_triple(&target_triple)
        .map_err(|err| BFError::LlvmError(format!("Failed to get target: `{}`", err)))?;
    let Some(target_machine) = target.create_target_machine(
        &target_triple,
        "generic",
        "",
        OptimizationLevel::None,
        RelocMode::PIC,
        CodeModel::Default,
    ) else {
        return Err(BFError::LlvmError(
            "Failed to initialize target machine".into(),
        ));
    };
    LLVMInstruction::build_instructions(&ctx, &optimized_instrs)?;

    ctx.builder
        .build_return(Some(&ctx.ctx.i32_type().const_zero()))
        .map_err(|_| BFError::LlvmError("Failed to build return from main".into()))?;

    ctx.module
        .verify()
        .map_err(|err| BFError::LlvmError(format!("Verification error: `{}`", err)))?;

    target_machine
        .write_to_file(&ctx.module, FileType::Object, args.output.as_path())
        .map_err(|err| BFError::LlvmError(format!("Failed to create object file: `{}`", err)))?;

    Ok(())
}
