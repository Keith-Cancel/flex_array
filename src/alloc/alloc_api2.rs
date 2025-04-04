use core::alloc::Layout;
use core::ptr::NonNull;

use allocator_api2::alloc::Allocator as Alloc2;

use super::AllocError;
use super::AltAllocator;

unsafe impl<A: Alloc2> AltAllocator for A {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let Ok(mem) = <Self as Alloc2>::allocate(self, layout) else {
            return Err(AllocError);
        };
        return Ok(mem);
    }

    #[inline]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let Ok(mem) = <Self as Alloc2>::allocate_zeroed(self, layout) else {
            return Err(AllocError);
        };
        return Ok(mem);
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { <Self as Alloc2>::deallocate(self, ptr, layout) };
    }

    #[inline]
    unsafe fn grow(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let Ok(mem) = (unsafe { <Self as Alloc2>::grow(self, old_ptr, old_layout, new_layout) }) else {
            return Err(AllocError);
        };
        return Ok(mem);
    }

    #[inline]
    unsafe fn grow_zeroed(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let Ok(mem) = (unsafe { <Self as Alloc2>::grow_zeroed(self, old_ptr, old_layout, new_layout) }) else {
            return Err(AllocError);
        };
        return Ok(mem);
    }
    #[inline]
    unsafe fn shrink(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let Ok(mem) = (unsafe { <Self as Alloc2>::shrink(self, old_ptr, old_layout, new_layout) }) else {
            return Err(AllocError);
        };
        return Ok(mem);
    }
}
