use std::alloc::{self, Layout};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

#[derive(Debug)]
pub struct Vector<T> {
    pointer: NonNull<T>,
    capacity: usize,
    length: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            0
        };
        Vector {
            pointer: NonNull::<T>::dangling(),
            capacity: cap,
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

    pub fn insert(&mut self, index: usize, element: T) {
        // <= because it is valid to insert an element after everything
        assert!(index <= self.length, "index is out of bounds");
        if self.length == self.capacity {
            self.grow();
        }
        unsafe {
            ptr::copy(
                self.pointer.as_ptr().add(index),
                self.pointer.as_ptr().add(index + 1),
                self.length - index,
            );
            ptr::write(self.pointer.as_ptr().add(index), element);
        }
        self.length += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.length, "index is out of bounds");
        self.length -= 1;
        unsafe {
            let result = ptr::read(self.pointer.as_ptr().add(index));
            ptr::copy(
                self.pointer.as_ptr().add(index + 1),
                self.pointer.as_ptr().add(index),
                self.length - index,
            );
            result
        }
    }

    fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow"); // ZST
        let (new_capacity, new_layout) = if self.capacity == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_capacity = (1.5 * self.capacity as f64).ceil() as usize;
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

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        let element_size = mem::size_of::<T>();
        if self.capacity != 0 && element_size != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.pointer.as_ptr() as *mut u8, layout);
            }
        }
    }
}

// Deref and DerefMut give us len, first, last, iter, mut and other slice related functions
impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.pointer.as_ptr(), self.length) }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length) }
    }
}

unsafe impl<T: Send> Send for Vector<T> {}
unsafe impl<T: Sync> Sync for Vector<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let mut v: Vector<i32> = Vector::new();
        v.push(10);
        assert!(v.len() == 1);
        v.push(-13);
        assert!(v.len() == 2);
    }

    #[test]
    fn test_first_last() {
        let mut v: Vector<i32> = Vector::new();
        v.push(10);
        v.push(15);
        v.push(-13);
        assert!(v.len() == 3);
        assert!(v.first() == Some(&10), "first failed");
        assert!(v.last() == Some(&-13), "last 1 failed");
        assert!(v.pop() == Some(-13), "-13 failed");
        assert!(v.last() == Some(&15), "last 2 failed");
    }

    #[test]
    fn test_pop() {
        let mut v: Vector<i32> = Vector::new();
        v.push(10);
        assert!(v.len() == 1);
        assert!(v.pop() == Some(10));
        assert!(v.len() == 0);
    }

    #[test]
    fn test_insert_remove() {
        let mut v: Vector<i32> = Vector::new();
        v.insert(0, 10);
        assert!(v.len() == 1);
        v.insert(1, -1);
        assert!(v.len() == 2);
        assert!(v.remove(0) == 10, "remove 10 from i 0 failed");
        assert!(v[0] == -1);
        assert!(v.len() == 1);
        assert!(v.remove(0) == -1, "remove -1 from i 0 failed");
        assert!(v.len() == 0);

        let mut v2: Vector<i32> = Vector::new();
        v2.push(10);
        v2.push(15);
        v2.push(-13);
        v2.insert(1, -100);
        assert!(v2.len() == 4);
        assert!(v2[0] == 10);
        assert!(v2[1] == -100);
        assert!(v2.remove(2) == 15);
        assert!(v2.len() == 3);
        assert!(v2[2] == -13);
    }

    #[test]
    fn test_zst() {
        // A is a zero sized type
        #[derive(PartialEq)]
        pub struct A(());
        let a: A = A(());

        let mut v: Vector<A> = Vector::new();
        assert!(v.len() == 0);
        v.push(a);
        assert!(v.len() == 1);
        assert!(v.pop() == Some(A(()))); // would make lispers proud
    }
}
