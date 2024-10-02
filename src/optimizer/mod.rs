//! BF program optimization passes

mod _coalesce;
pub use _coalesce::*;
mod _options;
pub use _options::*;

use crate::instruction::IntermediateInstruction;

/// Optimizes the given BF program according to the given options
pub fn optimize(instrs: Vec<IntermediateInstruction>, opts: OptimizerOptions) -> Vec<IntermediateInstruction> {
    let mut instrs = instrs;
    let mut changed = true;

    let optimizers = {
        let mut optimizers = vec![];
        if opts.coalesce { optimizers.push(coalesce); }
        optimizers
    };

    while changed {
        changed = false;
        for optimizer in &optimizers {
            let (new_instrs, new_changed) = optimizer(instrs);
            instrs = new_instrs;
            if new_changed { changed = true; }
        }
    }

    instrs
}
