use brainforge::generator::Architecture;
use brainforge::instruction::IntermediateInstruction;
use brainforge::jit::make_program;

fn main() {
    let memory: Vec<u8> = vec![0; 8192];
    let program = vec![
        IntermediateInstruction::Add(65),
        IntermediateInstruction::Write,
        IntermediateInstruction::Add(1),
        IntermediateInstruction::Write,
        IntermediateInstruction::Add(1),
        IntermediateInstruction::Write,
    ];
    let func = make_program(&*program, &Architecture::AMD64);
    func(memory.as_ptr() as *mut libc::c_void);
}
