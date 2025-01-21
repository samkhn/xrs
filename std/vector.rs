use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};

pub struct Vector<T> {
    pointer: NonNull<T>,
    capacity: usize,
    length: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(
            mem::size_of::<T>() != 0,
            "We aren't able to handle zero-sized types"
        );
        Vector {
            pointer: NonNull::<T>::dangling(),
            capacity: 0,
            length: 0,
        }
    }

    pub fn push(&mut self, element: T) {
        if self.length == self.capacity {
            self.grow();
        }
        unsafe {
            ptr::write(self.pointer.as_ptr().add(self.length), element);
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            unsafe { Some(ptr::read(self.pointer.as_ptr().add(self.length))) }
        }
    }

    fn grow(&mut self) {
        let (new_capacity, new_layout) = if self.capacity == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_capacity = (1.5 * self.capacity as f64) as usize;
            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            (new_capacity, new_layout)
        };

        // Why are we checking if less than isize (signed)?
        // (we normally index into arrays with unsigned ints)
        // Rust leans on LLVM GetElementPtr to do alias analysis of pointers
        // GetElementPtr (and thus std::ptr::offset) takes a signed integer
        // A more optimal way to check this is to check if PAE is available
        // If it isn't, we leave the assert.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation is too large"
        );

        // more about the allocator in alloc.rs and here
        // https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_llvm/src/allocator.rs
        let new_pointer = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_pointer = self.pointer.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_pointer, old_layout, new_layout.size()) }
        };

        self.pointer = match NonNull::new(new_pointer as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.capacity = new_capacity;
    }
}

unsafe impl<T: Send> Send for Vector<T> {}
unsafe impl<T: Sync> Sync for Vector<T> {}
