use std::mem;
use std::ptr;

use libc::c_void;

#[cfg(not(windows))]
use libc::{MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_WRITE};
#[cfg(windows)]
use winapi::um::{
    memoryapi::{VirtualAlloc, VirtualFree},
    winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE},
};

/// A JitFunction
/// Can be used to call the inner code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitFunction<T> {
    /// machine code which gets executed
    pub code: Vec<u8>,
    tmp: Vec<T>,
    mem: *mut c_void,
}

impl<T> JitFunction<T> {
    /// Creates new JitFunction
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code: code,
            tmp: vec![],
            mem: 0 as *mut c_void,
        }
    }

    /// Allocates the needed memory for the function
    pub unsafe fn alloc(&mut self) -> *mut c_void {
        let mem = alloc_executable_memory(self.code.len());
        if mem.is_null() {
            panic!("Error allocating memory")
        }

        ptr::copy_nonoverlapping(self.code.as_ptr(), mem as *mut u8, self.code.len());

        self.mem = mem;

        mem
    }

    /// Frees the allocated function memory
    pub unsafe fn free(&mut self) {
        dealloc_executable_memory(self.mem, self.code.len());
    }

    /// Changes the machine code of the function
    pub unsafe fn change(&mut self, new: Vec<u8>) {
        self.code = new;
        ptr::copy_nonoverlapping(self.code.as_ptr(), self.mem as *mut u8, self.code.len());
    }
}

#[cfg(not(windows))]
pub(crate) unsafe fn alloc_executable_memory(size: usize) -> *mut c_void {
    let ptr = libc::mmap(
        ptr::null_mut(),
        size,
        PROT_WRITE | PROT_EXEC,
        MAP_PRIVATE | MAP_ANON,
        -1,
        0,
    );
    if ptr == MAP_FAILED {
        ptr::null_mut()
    } else {
        ptr
    }
}

#[cfg(windows)]
pub(crate) unsafe fn alloc_executable_memory(size: usize) -> *mut c_void {
    VirtualAlloc(ptr::null_mut(), size, MEM_COMMIT, PAGE_EXECUTE_READWRITE) as *mut c_void
}

#[cfg(not(windows))]
pub(crate) unsafe fn dealloc_executable_memory(ptr: *mut c_void, size: usize) {
    libc::munmap(ptr, size);
}

#[cfg(windows)]
pub(crate) unsafe fn dealloc_executable_memory(ptr: *mut c_void, _size: usize) {
    VirtualFree(
        ptr as *mut winapi::ctypes::c_void,
        0,
        winapi::um::winnt::MEM_RELEASE,
    );
}

macro_rules! impl_unsafe_fn {
    (@recurse $first:ident $( , $rest:ident )*) => {
        impl_unsafe_fn!($( $rest ),*);
    };

    (@recurse) => {};

    ($( $param:ident ),*) => {
        impl<Output: Copy, $( $param ),*> JitFunction<unsafe extern "C" fn($( $param ),*) -> Output > {
            /// Calls the JitFunction with the given arguments
            /// ### Safty:
            /// This function calls into the jit erea which may lead to undefined context or segmentation faults
            #[allow(non_snake_case)]
            #[inline(always)]
            pub unsafe fn call(&mut self, $( $param: $param ),*) -> Output {
                let inner: unsafe extern "C" fn($( $param ),*) -> Output = mem::transmute(self.alloc());
                let out = (inner)($( $param ),*);
                let out = Box::new(out);
                let void_ptr: *const Output = &*out;
                self.free();
                *void_ptr
            }
        }

        impl_unsafe_fn!(@recurse $( $param ),*);
    };
}

impl_unsafe_fn!(
    A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21,
    A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36, A37, A38, A39, A40,
    A41, A42, A43, A44, A45, A46, A47, A48, A49, A50
);