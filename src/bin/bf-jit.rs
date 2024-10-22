use brainforge::generator::Architecture;
use brainforge::instruction::IntermediateInstruction;
use brainforge::jit::make_program;

fn main() {
    let mem_size = 8192;
    let memory: Vec<u8> = vec![0; mem_size];
    let program = vec![
        IntermediateInstruction::Add(65),
        IntermediateInstruction::Write,
        IntermediateInstruction::Add(1),
        IntermediateInstruction::Write,
        IntermediateInstruction::Add(1),
        IntermediateInstruction::Write,
    ];
    let func = make_program(&*program, &Architecture::AMD64);
    let tape_center =
        unsafe { memory.as_ptr().offset((mem_size / 2) as isize) as *mut libc::c_void };
    func(tape_center);
}
