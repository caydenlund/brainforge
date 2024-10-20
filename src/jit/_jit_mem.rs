use std::mem;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

pub const PAGE_SIZE: usize = 4096;

pub struct JitMem {
    pub contents: *mut u8,
    pub capacity: usize,
    pub size: usize,
}

impl JitMem {
    pub fn new(num_pages: usize) -> Self {
        let size = 0;
        let capacity = num_pages * PAGE_SIZE;

        let contents: *mut u8 = unsafe {
            let mut contents: MaybeUninit<*mut libc::c_void> = MaybeUninit::uninit();
            libc::posix_memalign(contents.as_mut_ptr(), PAGE_SIZE, capacity);
            let contents = contents.assume_init();
            libc::mprotect(
                contents,
                capacity,
                libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
            );

            mem::transmute(contents)
        };

        Self {
            contents,
            capacity,
            size,
        }
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: Iterator<Item = u8>,
    {
        for byte in iter {
            let size = self.size;
            if size + 1 >= self.capacity {
                panic!("Exceeded capacity of JIT memory");
            }

            self[size] = byte;
            self.size += 1;
        }
    }
}

impl Drop for JitMem {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.contents as *mut libc::c_void);
        }
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
