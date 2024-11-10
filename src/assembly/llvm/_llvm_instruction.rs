use crate::assembly::llvm::LlvmContext;
use crate::instruction::IntermediateInstruction;
use crate::{BFError, BFResult};
use inkwell::basic_block::BasicBlock;
use inkwell::types::BasicType;
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, BasicValueEnum, CallSiteValue, FunctionValue,
    InstructionValue, IntValue, PointerValue,
};
use inkwell::IntPredicate;

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

        fn load_mem_val<'c>(ctx: &'c LlvmContext) -> BFResult<IntValue<'c>> {
            let mem_val_ptr =
                load(ctx, "mem_val_ptr", ctx.mem_ptr.typ, ctx.mem_ptr.val)?.into_pointer_value();
            Ok(load(ctx, "mem_val", ctx.ctx.i8_type(), mem_val_ptr)?.into_int_value())
        }

        fn store_mem_val<'c, V: BasicValue<'c>>(
            ctx: &'c LlvmContext,
            val: V,
        ) -> BFResult<InstructionValue<'c>> {
            let mem_val_ptr =
                load(ctx, "mem_val_ptr", ctx.mem_ptr.typ, ctx.mem_ptr.val)?.into_pointer_value();
            store(ctx, "mem_val", mem_val_ptr, val)
        }

        fn shift_ptr<'c, T: BasicType<'c>>(
            ctx: &'c LlvmContext,
            name: &str,
            typ: T,
            ptr: PointerValue<'c>,
            offset: i32,
        ) -> BFResult<PointerValue<'c>> {
            unsafe {
                ctx.builder
                    .build_gep(
                        typ,
                        ptr,
                        &[ctx.ctx.i32_type().const_int(offset as u64, true)],
                        &format!("{}_shifted", name),
                    )
                    .map_err(|_| BFError::LlvmError(format!("Failed to move pointer `{}`", name)))
            }
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

        fn get_curr_fn<'c>(ctx: &'c LlvmContext) -> BFResult<FunctionValue<'c>> {
            let Some(bb_curr) = ctx.builder.get_insert_block() else {
                return Err(BFError::LlvmError("Builder is not in a basic block".into()));
            };
            let Some(fn_curr) = bb_curr.get_parent() else {
                return Err(BFError::LlvmError("Basic block has no parent".into()));
            };
            Ok(fn_curr)
        }

        let i8_val = |val: u64| ctx.ctx.i8_type().const_int(val, false);

        fn cond_branch<'c>(
            ctx: &'c LlvmContext,
            bb_zero: BasicBlock<'c>,
            bb_not_zero: BasicBlock<'c>,
        ) -> BFResult<InstructionValue<'c>> {
            let mem_val = load_mem_val(ctx)?;

            let branch_cond_loop = ctx
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    mem_val,
                    ctx.ctx.i8_type().const_zero(),
                    "loop_jmp_cond",
                )
                .map_err(|_| BFError::LlvmError("Failed to build loop jmp condition".into()))?;

            ctx.builder
                .build_conditional_branch(branch_cond_loop, bb_zero, bb_not_zero)
                .map_err(|_| BFError::LlvmError("Failed to build conditional branch".into()))
        }

        fn extend_i8_i32<'c>(
            ctx: &'c LlvmContext,
            name: &str,
            val: IntValue<'c>,
        ) -> BFResult<IntValue<'c>> {
            ctx.builder
                .build_int_z_extend(val, ctx.ctx.i32_type(), name)
                .map_err(|_| {
                    BFError::LlvmError(format!("Failed to build sign extend for `{}`", name))
                })
        }

        fn truncate_i32_i8<'c>(
            ctx: &'c LlvmContext,
            name: &str,
            val: IntValue<'c>,
        ) -> BFResult<IntValue<'c>> {
            ctx.builder
                .build_int_truncate(val, ctx.ctx.i8_type(), name)
                .map_err(|_| BFError::LlvmError(format!("Failed to build truncate for `{}`", name)))
        }

        fn int_add<'c>(
            ctx: &'c LlvmContext,
            name: &str,
            lhs: IntValue<'c>,
            rhs: IntValue<'c>,
        ) -> BFResult<IntValue<'c>> {
            ctx.builder
                .build_int_add(lhs, rhs, name)
                .map_err(|_| BFError::LlvmError(format!("Failed to build add for `{}`", name)))
        }

        match instr {
            IntermediateInstruction::Loop(sub_instrs) => {
                let fn_curr = get_curr_fn(ctx)?;

                // Branch past the end of the loop if the current cell holds 0,
                // or to the body otherwise
                let bb_loop_body = ctx.ctx.append_basic_block(fn_curr, "bb_loop_body");
                let bb_loop_end = ctx.ctx.append_basic_block(fn_curr, "bb_loop_end");
                cond_branch(ctx, bb_loop_end, bb_loop_body)?;

                // In the loop body, encode the sub-instructions and conditionally branch again
                ctx.builder.position_at_end(bb_loop_body);
                Self::build_instructions(ctx, sub_instrs)?;
                cond_branch(ctx, bb_loop_end, bb_loop_body)?;

                // Finally, at `bb_loop_end`, do nothing (future instructions will be added here)
                ctx.builder.position_at_end(bb_loop_end);
            }
            IntermediateInstruction::AddDynamic(target, multiplier) => {
                let mem_val = load_mem_val(ctx)?;
                let mem_val_i32 = extend_i8_i32(ctx, "mem_val_i32", mem_val)?;
                let product_val_i32 = ctx
                    .builder
                    .build_int_mul(
                        mem_val_i32,
                        ctx.ctx.i32_type().const_int(*multiplier as u64, true),
                        "product_val_i32",
                    )
                    .map_err(|_| BFError::LlvmError("Failed to build int multiply".into()))?;

                let mem_val_ptr = load(ctx, "mem_val_ptr", ctx.mem_ptr.typ, ctx.mem_ptr.val)?
                    .into_pointer_value();
                let dst_ptr =
                    shift_ptr(ctx, "mem_val_ptr", ctx.ctx.i8_type(), mem_val_ptr, *target)?;
                let dst_val = load(ctx, "dst_val", ctx.ctx.i8_type(), dst_ptr)?.into_int_value();
                let dst_val_i32 = extend_i8_i32(ctx, "dst_val_i32", dst_val)?;

                let sum_val_i32 = int_add(ctx, "sum_val", product_val_i32, dst_val_i32)?;
                let sum_val = truncate_i32_i8(ctx, "sum_val", sum_val_i32)?;
                store(ctx, "sum_val", dst_ptr, sum_val)?;
            }
            IntermediateInstruction::Zero => {
                store_mem_val(ctx, i8_val(0))?;
            }
            IntermediateInstruction::SimpleLoop(sub_instrs) => {
                let fn_curr = get_curr_fn(ctx)?;

                // Branch past the simple loop contents if the current cell holds 0
                let bb_loop_body = ctx.ctx.append_basic_block(fn_curr, "bb_loop_body");
                let bb_loop_end = ctx.ctx.append_basic_block(fn_curr, "bb_loop_end");
                cond_branch(ctx, bb_loop_end, bb_loop_body)?;

                // In the loop body, encode the sub-instructions and unconditionally branch
                ctx.builder.position_at_end(bb_loop_body);
                Self::build_instructions(ctx, sub_instrs)?;
                ctx.builder
                    .build_unconditional_branch(bb_loop_end)
                    .map_err(|_| {
                        BFError::LlvmError("Failed to build jump past loop contents".into())
                    })?;

                // Finally, at `bb_loop_end`, do nothing (future instructions will be added here)
                ctx.builder.position_at_end(bb_loop_end);
            }
            IntermediateInstruction::Move(stride) => {
                let mem_val_ptr = load(ctx, "mem_val_ptr", ctx.mem_ptr.typ, ctx.mem_ptr.val)?
                    .into_pointer_value();
                let mem_val_ptr_shifted =
                    shift_ptr(ctx, "mem_val_ptr", ctx.ctx.i8_type(), mem_val_ptr, *stride)?;
                store(ctx, "mem_val_ptr", ctx.mem_ptr.val, mem_val_ptr_shifted)?;
            }
            IntermediateInstruction::Add(offset) => {
                let mem_val = load_mem_val(ctx)?;
                let sum_val = int_add(ctx, "sum_val", mem_val, i8_val(*offset as u64))?;
                store_mem_val(ctx, sum_val)?;
            }
            IntermediateInstruction::Read => {
                let Some(ch_val) = call(ctx, "getchar", &[])?.try_as_basic_value().left() else {
                    return Err(BFError::LlvmError(
                        "Failed to get basic value from `getchar` call".into(),
                    ));
                };
                let ch_val_i8 = truncate_i32_i8(ctx, "ch_val_i8", ch_val.into_int_value())?;
                store_mem_val(ctx, ch_val_i8)?;
            }
            IntermediateInstruction::Write => {
                let mem_val = load_mem_val(ctx)?;
                let mem_val_i32 = extend_i8_i32(ctx, "mem_val_i32", mem_val)?;
                call(ctx, "putchar", &[mem_val_i32.into()])?;
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
