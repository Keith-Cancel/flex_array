pub use alloc_def::Global;

#[cfg(feature = "experimental_allocator")]
mod alloc_def {
    /// Re-export the std Global implementation of allocator APIs.
    pub use std::alloc::Global;
}

#[cfg(not(feature = "experimental_allocator"))]
mod alloc_def {
    use core::ptr;
    use core::ptr::NonNull;
    use std::alloc;
    use std::alloc::Layout;

    use crate::types::AllocError;
    use crate::types::AltAllocator;

    /// This is basically a wrapper around the std global allocator APIs.
    ///
    /// See:
    /// <https://doc.rust-lang.org/std/alloc/struct.Global.html>
    ///
    /// It has the same name as `Global` since the allocator APIs are
    /// not stabilized yet. When stabilized this will be just removed.
    /// Rust's `Global` will be exported for backwards compatibility.
    #[derive(Debug, Copy, Clone)]
    pub struct Global;

    unsafe impl AltAllocator for Global {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            // std::alloc::alloc() requires that the layout size be non-zero,
            // but the allocator API does not require this.
            if layout.size() == 0 {
                return Err(AllocError);
            };
            let ptr = unsafe { alloc::alloc(layout) };
            let Some(ptr) = NonNull::new(ptr) else {
                return Err(AllocError);
            };
            return Ok(NonNull::slice_from_raw_parts(ptr, layout.size()));
        }

        fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            if layout.size() == 0 {
                return Err(AllocError);
            };
            let ptr = unsafe { alloc::alloc_zeroed(layout) };
            let Some(ptr) = NonNull::new(ptr) else {
                return Err(AllocError);
            };
            return Ok(NonNull::slice_from_raw_parts(ptr, layout.size()));
        }

        unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: Layout) {
            unsafe { alloc::dealloc(ptr.as_ptr(), layout) };
        }

        unsafe fn grow(
            &self,
            old_ptr: NonNull<u8>,
            old_layout: Layout,
            new_layout: Layout,
        ) -> Result<NonNull<[u8]>, AllocError> {
            if new_layout.size() == 0 {
                return Err(AllocError);
            }

            let new = unsafe { alloc::realloc(old_ptr.as_ptr(), old_layout, new_layout.size()) };
            let Some(new) = NonNull::new(new) else {
                return Err(AllocError);
            };
            return Ok(NonNull::slice_from_raw_parts(new, new_layout.size()));
        }

        unsafe fn grow_zeroed(
            &self,
            old_ptr: NonNull<u8>,
            old_layout: Layout,
            new_layout: Layout,
        ) -> Result<NonNull<[u8]>, AllocError> {
            let old_sz = old_layout.size();
            let new_sz = new_layout.size();

            // In this case just allocate new zeroed memory.
            // that way any optimizations for `alloc::alloc_zeroed()`
            // are used.
            if old_sz == 0 {
                return self.allocate_zeroed(new_layout);
            }

            // Do nothing in this case.
            // This also means that if it's not true
            // new_sz is greater than zero.
            if new_sz <= old_sz {
                return Ok(NonNull::slice_from_raw_parts(old_ptr, old_layout.size()));
            }

            let new = unsafe { alloc::realloc(old_ptr.as_ptr(), old_layout, new_layout.size()) };
            let Some(new) = NonNull::new(new) else {
                return Err(AllocError);
            };

            // Move to end of the old data.
            let start = unsafe { new.add(old_sz) };
            unsafe {
                start.write_bytes(0, new_sz - old_sz);
            };

            return Ok(NonNull::slice_from_raw_parts(new, new_layout.size()));
        }

        unsafe fn shrink(
            &self,
            old_ptr: NonNull<u8>,
            old_layout: Layout,
            new_layout: Layout,
        ) -> Result<NonNull<[u8]>, AllocError> {
            if new_layout.size() == 0 {
                return Err(AllocError);
            }
            let new = unsafe { alloc::realloc(old_ptr.as_ptr(), old_layout, new_layout.size()) };
            let Some(new) = NonNull::new(new) else {
                return Err(AllocError);
            };
            return Ok(NonNull::slice_from_raw_parts(new, new_layout.size()));
        }
    }
}
