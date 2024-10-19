extern crate libc;

use libc::c_int;
use std::mem::{self, MaybeUninit};
use std::ops::{Index, IndexMut};

const PAGE_SIZE: usize = 4096;

struct JitMem {
    contents: *mut u8,
}

impl JitMem {
    fn new(num_pages: usize) -> JitMem {
        let size = num_pages * PAGE_SIZE;

        let contents: *mut u8 = unsafe {
            let mut contents: MaybeUninit<*mut libc::c_void> = MaybeUninit::uninit();
            libc::posix_memalign(contents.as_mut_ptr(), PAGE_SIZE, size);
            let contents = contents.assume_init();
            libc::mprotect(
                contents,
                size,
                libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
            );

            mem::transmute(contents)
        };

        JitMem { contents }
    }
}

impl Index<usize> for JitMem {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.contents.offset(index as isize) }
    }
}

impl IndexMut<usize> for JitMem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.contents.offset(index as isize) }
    }
}

fn mk_func() -> fn(unsafe extern "C" fn() -> c_int, unsafe extern "C" fn(c_int) -> c_int) {
    let mut program = JitMem::new(1);

    // mov %rdi,%r14
    program[0] = 0x49;
    program[1] = 0x89;
    program[2] = 0xfe;

    // mov %rsi,%r15
    program[3] = 0x49;
    program[4] = 0x89;
    program[5] = 0xf7;

    // call *%r14
    program[6] = 0x41;
    program[7] = 0xff;
    program[8] = 0xd6;

    // mov %eax,%esi
    program[9] = 0x89;
    program[10] = 0xc7;

    // call *%r15
    program[11] = 0x41;
    program[12] = 0xff;
    program[13] = 0xd7;

    // ret
    program[14] = 0xC3;

    unsafe { mem::transmute(program.contents) }
}

fn main() {
    let func = mk_func();
    func(libc::getchar, libc::putchar);
}
