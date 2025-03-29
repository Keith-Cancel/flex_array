use core::alloc::Layout;
use core::ptr::NonNull;

#[cfg(feature = "experimental_allocator")]
pub use core::alloc::AllocError;
#[cfg(feature = "experimental_allocator")]
use core::alloc::Allocator;

/// This indicates some sort of memory allocation error for the alt allocator.
///
/// If the rust allocator API is enabled this will be the same error type as
/// the Allocator API.
#[cfg(not(feature = "experimental_allocator"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AllocError;

/// The rust allocator API is not stable yet. Therefore, this trait
/// can be used to implement/wrap a custom allocator in a no_std environment.
/// It mirrors the unstable allocator API at the moment.
///
/// This mirrors the safety requirements of the allocator API:
/// https://doc.rust-lang.org/std/alloc/trait.Allocator.html
///
///
/// If the allocator api is ever marked as stable this trait probably can
/// be removed.
pub unsafe trait AltAllocator {
    /// Allocates a chunk of memory with the given layout.
    ///
    /// On success it returns a pointer to the allocated memory.
    ///
    /// If the allocation fails or has some kinda of error it will return
    /// an `AllocError`.
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;

    /// Allocates just like `allocate` but also zeroes the memory.
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ret = self.allocate(layout)?;
        let ptr = ret.cast::<u8>();
        unsafe { ptr.write_bytes(0, ret.len()) };
        return Ok(ret);
    }

    /// Deallocates the chunk of memory pointed at by`ptr`
    ///
    /// This memory must have only been allocated by this allocator.
    /// The layout must match the layout provided when the chunk was
    /// allocated.
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);

    /// Grows the memory pointed at by `old_ptr` to the new layout.
    ///
    /// The new layout must be larger than the old layout.
    ///
    /// If this fails the old ptr must still will be valid. If it succeeds
    /// the old ptr is not longer valid, and the ptr returned must be used
    /// instead.
    unsafe fn grow(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new = self.allocate(new_layout)?;
        let ptr = new.cast::<u8>();

        // Copy the old data to the new location
        unsafe { ptr.copy_from_nonoverlapping(old_ptr, old_layout.size()) };
        // free the old memory
        unsafe { self.deallocate(old_ptr, old_layout) };
        return Ok(new);
    }

    /// Behaves just like `grow` but the new memory will be zeroed.
    unsafe fn grow_zeroed(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new = self.allocate_zeroed(new_layout)?;
        let ptr = new.cast::<u8>();

        // Copy the old data to the new location
        unsafe { ptr.copy_from_nonoverlapping(old_ptr, old_layout.size()) };
        // free the old memory
        unsafe { self.deallocate(old_ptr, old_layout) };
        return Ok(new);
    }

    /// Shrinks the memory pointed at by `old_ptr` to the new layout.
    /// The new layout must be smaller than the old layout.
    ///
    /// If this fails the old ptr will still be valid. If it succeeds
    /// the old ptr is not longer valid, and the ptr returned must be used
    /// instead.
    unsafe fn shrink(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new = self.allocate(new_layout)?;
        let ptr = new.cast::<u8>();

        // Copy the old data to the new location
        unsafe { ptr.copy_from_nonoverlapping(old_ptr, new_layout.size()) };
        // free the old memory
        unsafe { self.deallocate(old_ptr, old_layout) };
        return Ok(new);
    }
}


#[cfg(feature = "experimental_allocator")]
unsafe impl<A> AltAllocator for A
where
    A: Allocator,
{
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        return <Self as Allocator>::allocate(self, layout);
    }
    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { <Self as Allocator>::deallocate(self, ptr, layout) };
    }
    #[inline]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        return <Self as Allocator>::allocate_zeroed(self, layout);
    }
    #[inline]
    unsafe fn grow(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        return unsafe { <Self as Allocator>::grow(self, old_ptr, old_layout, new_layout) };
    }
    #[inline]
    unsafe fn grow_zeroed(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        return unsafe { <Self as Allocator>::grow_zeroed(self, old_ptr, old_layout, new_layout) };
    }
    #[inline]
    unsafe fn shrink(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        return unsafe { <Self as Allocator>::shrink(self, old_ptr, old_layout, new_layout) };
    }
}
