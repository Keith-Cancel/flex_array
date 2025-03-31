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
fn inner_new() {
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
fn array_new() {
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
fn push_fail() {
    let mut arr = FlexArr::<u32, NoAlloc>::new_in(NoAlloc);
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.capacity(), 0);

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

#[test]
fn reserve_fail() {
    let mut arr = FlexArr::<u32, NoAlloc, u8>::new_in(NoAlloc);
    assert!(arr.reserve(0).is_ok());

    let err = arr.reserve(1);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::AllocFailure);
    }

    let err = arr.reserve_exact(1);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::AllocFailure);
    }

    let err = arr.reserve_usize(1);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::AllocFailure);
    }

    let err = arr.reserve_usize(256);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::CapacityOverflow);
    }

    let mut arr = FlexArr::<(), NoAlloc, u8>::new_in(NoAlloc);
    assert!(arr.push(()).is_ok());

    let err = arr.reserve(255);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::CapacityOverflow);
    }

    let err = arr.reserve_exact(255);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::CapacityOverflow);
    }
}

#[cfg(feature = "std_alloc")]
mod std_alloc {
    use core::cell::Cell;
    use std::string::String;
    use std::string::ToString;

    use super::*;
    use crate::types::Global;

    struct AllocCount(u8, Cell<u8>);

    impl AllocCount {
        const fn new(limit: u8) -> Self {
            return Self(limit, Cell::new(0));
        }
    }

    unsafe impl AltAllocator for AllocCount {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            let cur = self.1.get();
            if cur >= self.0 {
                return Err(AllocError);
            };
            self.1.set(cur + 1);
            return Global.allocate(layout);
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unsafe { Global.deallocate(ptr, layout) };
        }
    }

    #[test]
    fn double_reserve() {
        let mut arr = FlexArr::<u8, AllocCount>::new_in(AllocCount::new(1));
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), 0);

        let err = arr.reserve(1);
        assert!(err.is_ok());

        let err = arr.reserve_exact(1);
        assert!(err.is_ok());

        let err = arr.reserve_exact(1024);
        assert!(err.is_err());
        if let Err(e) = err {
            assert_eq!(e.reason(), ErrorReason::AllocFailure);
        }
    }

    #[test]
    fn push_pop() {
        let mut arr = FlexArr::<u8>::new();
        assert_eq!(size_of_val(&arr), size_of::<ExpectedSizeU32>());

        arr.push(0xcu8).unwrap();
        arr.push(0xau8).unwrap();
        arr.push(0xfu8).unwrap();
        arr.push(0xeu8).unwrap();

        assert_eq!(arr.len(), 4);

        assert_eq!(arr[0u32], 0xc);
        assert_eq!(arr[1u32], 0xa);
        assert_eq!(arr[2u32], 0xf);
        assert_eq!(arr[3u32], 0xe);

        assert_eq!(arr.pop().unwrap(), 0xeu8);

        arr.push(127).unwrap();
        assert_eq!(arr[3], 127);

        arr[0] = 0x99;

        assert_eq!(arr.pop().unwrap(), 127);
        assert_eq!(arr.pop().unwrap(), 0xf);
        assert_eq!(arr.pop().unwrap(), 0xa);
        assert_eq!(arr.pop().unwrap(), 0x99);
        assert!(arr.pop().is_none());

        let mut arr = FlexArr::<String>::with_capacity(2).unwrap();
        arr.push("Hello".to_string()).unwrap();
        arr.push("There".to_string()).unwrap();
        assert_eq!(arr[0], "Hello");
        assert_eq!(arr[1], "There");

        let there = arr.pop().unwrap();
        assert_eq!(there, "There");
    }

    #[test]
    fn usize_and_layout_failure() {
        let massive: u128 = (usize::MAX as u128) + 1;
        let ret = FlexArr::<u8, Global, u128>::with_capacity_in(Global, massive);
        assert!(ret.is_err());
        if let Err(e) = ret {
            assert_eq!(e.reason(), ErrorReason::UsizeOverflow);
        }

        let massive: u128 = (isize::MAX as u128) + 1;
        let ret = FlexArr::<u8, Global, u128>::with_capacity_in(Global, massive);
        assert!(ret.is_err());
        if let Err(e) = ret {
            assert_eq!(e.reason(), ErrorReason::LayoutFailure);
        }

        let massive = (usize::MAX / 256) + 1;
        let ret = FlexArr::<[u8; 256], Global, usize>::with_capacity_in(Global, massive);
        assert!(ret.is_err());
        if let Err(e) = ret {
            assert_eq!(e.reason(), ErrorReason::UsizeOverflow);
        }

        let massive = ((isize::MAX / 256) + 1) as usize;
        let ret = FlexArr::<[u8; 256], Global, usize>::with_capacity_in(Global, massive);
        assert!(ret.is_err());
        if let Err(e) = ret {
            assert_eq!(e.reason(), ErrorReason::LayoutFailure);
        }
    }
}
