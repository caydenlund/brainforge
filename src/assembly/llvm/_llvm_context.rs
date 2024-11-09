use crate::{BFError, BFResult};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{ArrayType, BasicType, FunctionType, PointerType};
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::AddressSpace;
use std::collections::HashMap;

pub struct LlvmValue<T, V> {
    pub typ: T,
    pub val: V,
}

pub struct LlvmFn<'c> {
    pub typ: FunctionType<'c>,
    pub val: FunctionValue<'c>,
    pub blocks: Option<Vec<BasicBlock<'c>>>,
}

pub struct LlvmContext<'c> {
    pub ctx: &'c Context,
    pub module: Module<'c>,
    pub builder: Builder<'c>,

    pub fns: HashMap<String, LlvmFn<'c>>,

    pub mem: LlvmValue<ArrayType<'c>, PointerValue<'c>>,
    pub mem_ptr: LlvmValue<PointerType<'c>, PointerValue<'c>>,
}

impl<'c> LlvmContext<'c> {
    pub fn new(ctx: &'c Context, mem_size: usize) -> BFResult<Self> {
        let module = ctx.create_module("mod_bf");
        let builder = ctx.create_builder();

        let mut fns = HashMap::new();

        let fn_putchar = {
            let typ = ctx.i32_type().fn_type(&[ctx.i32_type().into()], false);
            module.add_function("putchar", typ, None);
            let Some(val) = module.get_function("putchar") else {
                return Err(BFError::LlvmError(
                    "Failed to get function `putchar` from the module".into(),
                ));
            };
            let blocks = None;
            LlvmFn { typ, val, blocks }
        };
        fns.insert("putchar".into(), fn_putchar);

        let fn_main = {
            let typ = ctx.i32_type().fn_type(&[], false);
            let val = module.add_function("main", typ, None);
            let bb_main_entry = ctx.append_basic_block(val, "bb_main_entry");
            builder.position_at_end(bb_main_entry);
            let blocks = Some(vec![bb_main_entry]);
            LlvmFn { typ, val, blocks }
        };
        fns.insert("main".into(), fn_main);

        let mem = {
            let typ = ctx.i8_type().array_type(mem_size as u32);
            let val = builder
                .build_alloca(typ, "mem")
                .map_err(|_| BFError::LlvmError("Failed to build `mem` array allocation".into()))?;

            builder
                .build_memset(
                    val,
                    1,
                    ctx.i8_type().const_zero(),
                    ctx.i32_type().const_int(mem_size as u64, false),
                )
                .map_err(|_| {
                    BFError::LlvmError("Failed to build `memset` for `mem` initialization".into())
                })?;

            LlvmValue { typ, val }
        };

        let mem_ptr = {
            let typ = ctx.ptr_type(AddressSpace::default());
            let val = builder
                .build_alloca(typ, "mem_ptr")
                .map_err(|_| BFError::LlvmError("Failed to build `mem` array allocation".into()))?;

            unsafe {
                builder
                    .build_gep(
                        ctx.i8_type(),
                        mem.val,
                        &[ctx.i32_type().const_int(mem_size as u64 / 2, false)],
                        "mem_ptr",
                    )
                    .map_err(|_| {
                        BFError::LlvmError("Failed to build initial `gep` for `mem_ptr`".into())
                    })?;
            }

            LlvmValue { typ, val }
        };

        Ok(Self {
            ctx,
            module,
            builder,

            fns,

            mem,
            mem_ptr,
        })
    }
}
