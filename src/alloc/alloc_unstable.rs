use core::alloc::Allocator;
use core::alloc::Layout;
use core::ptr::NonNull;

use super::AllocError;
use super::AltAllocator;

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
