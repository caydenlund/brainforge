use crate::assembly::llvm::LlvmContext;
use crate::instruction::IntermediateInstruction;
use crate::{BFError, BFResult};

#[derive(Clone, Debug)]
pub enum LLVMInstruction {}

impl LLVMInstruction {
    fn build_instruction(ctx: &LlvmContext, instr: &IntermediateInstruction) -> BFResult<()> {
        match instr {
            IntermediateInstruction::Loop(sub_instrs) => {
                let curr_fn = ctx
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();
                let bb_loop_cond = ctx.ctx.append_basic_block(curr_fn, "bb_loop_cond");
                ctx.builder
                    .build_unconditional_branch(bb_loop_cond)
                    .map_err(|_| {
                        BFError::LlvmError(
                            "Failed to build unconditional branch to loop condition".into(),
                        )
                    })?;
                ctx.builder.position_at_end(bb_loop_cond);

                let bb_loop_body = ctx.ctx.append_basic_block(curr_fn, "bb_loop_body");
                let bb_loop_end = ctx.ctx.append_basic_block(curr_fn, "bb_loop_end");
                todo!() //
            }
            IntermediateInstruction::AddDynamic(_, _) => {
                todo!() //
            }
            IntermediateInstruction::Zero => {
                todo!() //
            }
            IntermediateInstruction::SimpleLoop(_) => {
                todo!() //
            }
            IntermediateInstruction::Move(_) => {
                todo!() //
            }
            IntermediateInstruction::Add(offset) => {
                let mem_val = ctx
                    .builder
                    .build_load(ctx.ctx.i8_type(), ctx.mem_ptr.val, "mem_val")
                    .map_err(|_| BFError::LlvmError("Failed to build load from `mem_ptr`".into()))?
                    .into_int_value();
                let sum = ctx
                    .builder
                    .build_int_add(
                        mem_val,
                        ctx.ctx.i8_type().const_int(*offset as u64, false),
                        "sum",
                    )
                    .map_err(|_| BFError::LlvmError("Failed to build add to `mem_val`".into()))?;
                ctx.builder
                    .build_store(ctx.mem_ptr.val, sum)
                    .map_err(|_| BFError::LlvmError("Failed to build store to `mem_ptr`".into()))?;
            }
            IntermediateInstruction::Read => {
                todo!() //
            }
            IntermediateInstruction::Write => {
                let mem_val = ctx
                    .builder
                    .build_load(ctx.ctx.i8_type(), ctx.mem_ptr.val, "mem_val")
                    .map_err(|_| BFError::LlvmError("Failed to build load from `mem_ptr`".into()))?
                    .into_int_value();
                let mem_val_32 = ctx
                    .builder
                    .build_int_z_extend(mem_val, ctx.ctx.i32_type(), "mem_val_32")
                    .map_err(|_| {
                        BFError::LlvmError("Failed to build sign extend for `mem_val`".into())
                    })?;
                ctx.builder
                    .build_call(
                        ctx.fns["putchar"].val,
                        &[mem_val_32.into()],
                        "fn_putchar_call",
                    )
                    .map_err(|_| BFError::LlvmError("Failed to build call to `putchar`".into()))?;
            }
            IntermediateInstruction::Scan(_) => {
                todo!() //
            }
        }
        Ok(())
    }

    pub fn build_instructions(
        ctx: &LlvmContext,
        instrs: &[IntermediateInstruction],
    ) -> BFResult<()> {
        for instr in instrs {
            Self::build_instruction(ctx, instr)?;
        }
        Ok(())
    }
}
