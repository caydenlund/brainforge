use crate::assembly::llvm::LlvmContext;
use crate::instruction::IntermediateInstruction;
use crate::{BFError, BFResult};
use inkwell::types::BasicType;
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, BasicValueEnum, CallSiteValue, InstructionValue,
    PointerValue,
};

#[derive(Clone, Debug)]
pub enum LLVMInstruction {}

impl LLVMInstruction {
    fn build_instruction(ctx: &LlvmContext, instr: &IntermediateInstruction) -> BFResult<()> {
        fn load<'c, T: BasicType<'c>>(
            ctx: &'c LlvmContext,
            name: &str,
            typ: T,
            ptr: PointerValue<'c>,
        ) -> BFResult<BasicValueEnum<'c>> {
            ctx.builder
                .build_load(typ, ptr, name)
                .map_err(|_| BFError::LlvmError(format!("Failed to build load from `{}`", name)))
        }

        fn store<'c, T: BasicValue<'c>>(
            ctx: &'c LlvmContext,
            name: &str,
            ptr: PointerValue<'c>,
            val: T,
        ) -> BFResult<InstructionValue<'c>> {
            ctx.builder
                .build_store(ptr, val)
                .map_err(|_| BFError::LlvmError(format!("Failed to build store to `{}`", name)))
        }

        fn load_mem_val<'c>(ctx: &'c LlvmContext) -> BFResult<BasicValueEnum<'c>> {
            let ptr =
                load(ctx, "mem_val_ptr", ctx.mem_ptr.typ, ctx.mem_ptr.val)?.into_pointer_value();
            load(ctx, "mem_val", ctx.ctx.i8_type(), ptr)
        }

        fn store_mem_val<'c, V: BasicValue<'c>>(
            ctx: &'c LlvmContext,
            val: V,
        ) -> BFResult<InstructionValue<'c>> {
            let ptr =
                load(ctx, "mem_val_ptr", ctx.mem_ptr.typ, ctx.mem_ptr.val)?.into_pointer_value();
            store(ctx, "mem_val", ptr, val)
        }

        fn call<'c>(
            ctx: &'c LlvmContext,
            name: &str,
            args: &[BasicMetadataValueEnum<'c>],
        ) -> Result<CallSiteValue<'c>, BFError> {
            let Some(fn_entry) = ctx.fns.get(name) else {
                return Err(BFError::LlvmError(format!(
                    "Failed to find entry for fn `{}`",
                    name
                )));
            };
            ctx.builder
                .build_call(fn_entry.val, args, &format!("fn_{}_call", name))
                .map_err(|_| BFError::LlvmError(format!("Failed to build call to `{}`", name)))
        }

        let i8_val = |val: u64| ctx.ctx.i8_type().const_int(val, false);

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
                store_mem_val(ctx, i8_val(0))?;
            }
            IntermediateInstruction::SimpleLoop(_) => {
                todo!() //
            }
            IntermediateInstruction::Move(_) => {
                todo!() //
            }
            IntermediateInstruction::Add(offset) => {
                let mem_val = load_mem_val(ctx)?;
                let sum = ctx
                    .builder
                    .build_int_add(mem_val.into_int_value(), i8_val(*offset as u64), "sum")
                    .map_err(|_| BFError::LlvmError("Failed to build add to `mem_val`".into()))?;
                store_mem_val(ctx, sum)?;
            }
            IntermediateInstruction::Read => {
                let Some(ch) = call(ctx, "getchar", &[])?.try_as_basic_value().left() else {
                    return Err(BFError::LlvmError(
                        "Failed to get basic value from `getchar` call".into(),
                    ));
                };
                let ch_val_8 = ctx
                    .builder
                    .build_int_truncate(ch.into_int_value(), ctx.ctx.i8_type(), "ch_val_8")
                    .map_err(|_| {
                        BFError::LlvmError(
                            "Failed to build `getchar` result truncation to char".into(),
                        )
                    })?;
                store_mem_val(ctx, ch_val_8)?;
            }
            IntermediateInstruction::Write => {
                let mem_val = load_mem_val(ctx)?.into_int_value();
                let mem_val_32 = ctx
                    .builder
                    .build_int_z_extend(mem_val, ctx.ctx.i32_type(), "mem_val_32")
                    .map_err(|_| {
                        BFError::LlvmError("Failed to build sign extend for `mem_val`".into())
                    })?;
                call(ctx, "putchar", &[mem_val_32.into()])?;
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
