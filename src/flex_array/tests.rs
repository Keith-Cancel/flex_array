use core::alloc::Layout;
use core::mem::size_of;
use core::mem::size_of_val;
use core::ptr::NonNull;
use core::ptr::dangling_mut;

use super::FlexArr;
use super::inner::Inner;
use crate::types::AllocError;
use crate::types::AltAllocator;
use crate::types::ErrorReason;

struct NoAlloc;

unsafe impl AltAllocator for NoAlloc {
    fn allocate(&self, _: Layout) -> Result<NonNull<[u8]>, AllocError> {
        return Err(AllocError);
    }
    unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
        return;
    }
}

struct ExpectedSizeU32 {
    _p: NonNull<u8>,
    _a: u32,
    _b: u32,
}

struct ExpectedSizeU16 {
    _p: NonNull<u8>,
    _a: u16,
    _b: u16,
}

struct ExpectedSizeU8 {
    _p: NonNull<u8>,
    _a: u8,
    _b: u8,
}

#[test]
fn test_inner_new() {
    // Check for index type `u32`(default) and for the type `u32`
    let inner = Inner::<NoAlloc>::new_in::<u32>(NoAlloc);
    assert_eq!(inner.capacity(size_of::<u32>()), 0);
    assert_eq!(inner.length, 0);
    assert_eq!(size_of_val(&inner), size_of::<ExpectedSizeU32>());
    assert_eq!(inner.get_ptr(), dangling_mut::<u32>());

    // Check for index type u16 and for the type `u64`
    let inner = Inner::<NoAlloc, u16>::new_in::<u64>(NoAlloc);
    assert_eq!(inner.capacity(size_of::<u64>()), 0);
    assert_eq!(inner.length, 0);
    assert_eq!(size_of_val(&inner), size_of::<ExpectedSizeU16>());
    assert_eq!(inner.get_ptr(), dangling_mut::<u64>());

    // Check for index type u8 and for the type `()`
    let inner = Inner::<NoAlloc, u8>::new_in::<()>(NoAlloc);
    assert_eq!(inner.capacity(size_of::<()>()), u8::MAX);
    assert_eq!(inner.length, 0);
    assert_eq!(size_of_val(&inner), size_of::<ExpectedSizeU8>());
    assert_eq!(inner.get_ptr(), dangling_mut::<()>());
}

#[test]
fn test_array_new() {
    // u32 length and u32 type
    let arr = FlexArr::<u32, NoAlloc>::new_in(NoAlloc);
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), 0);
    assert_eq!(size_of_val(&arr), size_of::<ExpectedSizeU32>());

    // u16 length and u64 type
    let arr = FlexArr::<u64, NoAlloc, u16>::new_in(NoAlloc);
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), 0);
    assert_eq!(size_of_val(&arr), size_of::<ExpectedSizeU16>());

    // u8 length and `()`type
    let arr = FlexArr::<(), NoAlloc, u8>::new_in(NoAlloc);
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), u8::MAX);
    assert_eq!(size_of_val(&arr), size_of::<ExpectedSizeU8>());
}

#[test]
fn test_no_memory() {
    let mut arr = FlexArr::<u32, NoAlloc>::new_in(NoAlloc);
    assert_eq!(arr.len(), 0);

    // This should fail
    let ret = arr.push(0);
    assert!(ret.is_err());
    if let Err(e) = ret {
        assert_eq!(e.reason(), ErrorReason::AllocFailure)
    }

    let mut arr = FlexArr::<(), NoAlloc, u8>::new_in(NoAlloc);
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), u8::MAX);

    // I should be able to push this ZST 255 times.
    for _ in 0..u8::MAX {
        assert!(arr.push(()).is_ok());
    }
    assert_eq!(arr.len(), u8::MAX);

    // This should fail
    let ret = arr.push(());
    assert!(ret.is_err());

    if let Err(e) = ret {
        assert_eq!(e.reason(), ErrorReason::CapacityOverflow)
    }
}
