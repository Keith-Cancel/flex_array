use core::alloc::Layout;
use core::mem::size_of;
use core::mem::size_of_val;
use core::ptr::NonNull;
use core::ptr::dangling_mut;

use super::FlexArr;
use super::inner::Inner;
use crate::alloc::AllocError;
use crate::alloc::AltAllocator;
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

#[test]
fn extend_from_slice_fail() {
    let mut arr = FlexArr::<u32, NoAlloc>::new_in(NoAlloc);
    let err = arr.extend_from_slice(&[0u32]);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::AllocFailure);
    }
    let mut arr = FlexArr::<(), NoAlloc, u8>::new_in(NoAlloc);

    let data = [(); 255];
    assert!(arr.extend_from_slice(&data).is_ok());

    let err = arr.extend_from_slice(&data);
    assert!(err.is_err());
    if let Err(e) = err {
        assert_eq!(e.reason(), ErrorReason::CapacityOverflow);
    }
}

#[cfg(feature = "std_alloc")]
mod std_alloc {
    use core::cell::Cell;
    use core::panic;
    use std::string::String;
    use std::string::ToString;

    use super::*;
    use crate::alloc::Global;

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
        assert_eq!(arr.last(), None);

        arr.push(0xcu8).unwrap();
        arr.push(0xau8).unwrap();
        arr.push(0xfu8).unwrap();
        arr.push(0xeu8).unwrap();

        assert_eq!(arr.len(), 4);
        assert_eq!(arr.last(), Some(&0xeu8));
        assert_eq!(arr.last_mut(), Some(&mut 0xeu8));

        assert_eq!(arr[0u32], 0xc);
        assert_eq!(arr.get(0), Some(&0xcu8));
        assert_eq!(arr.get_mut(0), Some(&mut 0xcu8));
        assert_eq!(arr[1u32], 0xa);
        assert_eq!(arr.get(1), Some(&0xau8));
        assert_eq!(arr[2u32], 0xf);
        assert_eq!(arr.get(2), Some(&0xfu8));
        assert_eq!(arr[3u32], 0xe);
        assert_eq!(arr.get(3), Some(&0xeu8));
        assert_eq!(arr.get(4), None);
        assert_eq!(arr.get_mut(4), None);

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

    /// Note: This test assumes usize is smaller than u128
    /// Likely, true assumption for most any architectures.
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

    #[test]
    fn add_slice() {
        let data: [u8; 5] = [1, 2, 3, 4, 5];
        let mut arr = FlexArr::<u8>::new();
        assert!(arr.push(10).is_ok());
        assert_eq!(arr.len(), 1);
        assert!(arr.capacity() >= 1);

        assert!(arr.extend_from_slice(&data).is_ok());
        assert_eq!(arr.len(), 6);
        assert!(arr.capacity() >= 6);

        assert_eq!(arr[0], 10);
        for i in 1..6 {
            assert_eq!(arr[i], data[(i - 1) as usize]);
        }

        let mut arr = FlexArr::<u8, AllocCount>::new_in(AllocCount::new(1));
        assert!(arr.reserve_exact(1).is_ok());
        assert!(arr.capacity() == 1);

        assert!(arr.push(10).is_ok());
        assert_eq!(arr.len(), 1);
        assert!(arr.capacity() == 1);

        let ret = arr.extend_from_slice(&data);
        assert!(ret.is_err());
        if let Err(e) = ret {
            assert_eq!(e.reason(), ErrorReason::AllocFailure);
        }
    }

    /// Note: This test assumes usize is smaller than u128
    /// Likely, true assumption for most any architectures.
    #[test]
    fn massive_slice() {
        // Rust's vec allows for Zero sized types to go to
        // usize::MAX. Even though normally this is limited
        // to isize::MAX.
        let data = [(); usize::MAX];
        let mut arr = std::vec::Vec::<()>::new();
        assert!(arr.capacity() == usize::MAX);
        arr.extend_from_slice(&data);
        assert_eq!(arr.len(), usize::MAX);

        // Make sure that FlexArr can behave like this for
        // even bigger types.
        let mut arr = FlexArr::<(), Global, u128>::new();
        assert!(arr.capacity() == u128::MAX);
        assert!(arr.extend_from_slice(&data).is_ok());
        assert_eq!(arr.len(), usize::MAX as u128);

        // Still should succeed even though we are beyond
        // both usize::MAX and isize::MAX now.
        assert!(arr.push(()).is_ok());
        assert_eq!(arr.len(), (usize::MAX as u128) + 1);
    }

    #[test]
    #[should_panic]
    fn massive_index_failure1() {
        let data = [(); usize::MAX];
        let mut arr = FlexArr::<(), Global, u128>::new();
        assert!(arr.capacity() == u128::MAX);

        assert!(arr.extend_from_slice(&data).is_ok());
        assert!(arr.push(()).is_ok());
        assert!(arr.push(()).is_ok());

        let index = usize::MAX as u128 + 1;
        let _ = arr[index];
    }

    #[test]
    #[should_panic]
    fn massive_index_failure2() {
        let data = [(); usize::MAX];
        let mut arr = FlexArr::<(), Global, u128>::new();
        assert!(arr.capacity() == u128::MAX);

        assert!(arr.extend_from_slice(&data).is_ok());
        assert!(arr.push(()).is_ok());
        assert!(arr.push(()).is_ok());

        let index = usize::MAX as u128 + 1;
        arr[index] = ();
    }

    #[test]
    fn swap_remove() {
        let mut arr = FlexArr::<String>::new();
        assert!(arr.swap_remove(0).is_none());

        arr.push("Hello".to_string()).unwrap();
        arr.push("There".to_string()).unwrap();
        arr.push("It is a beautiful day".to_string()).unwrap();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], "Hello");
        assert_eq!(arr[1], "There");
        assert_eq!(arr[2], "It is a beautiful day");

        assert_eq!(arr.swap_remove(0).unwrap(), "Hello");

        assert_eq!(arr[0], "It is a beautiful day");
        assert_eq!(arr[1], "There");
    }

    #[test]
    fn remove() {
        let mut arr = FlexArr::<String>::new();
        assert!(arr.remove(0).is_none());

        arr.push("Hello".to_string()).unwrap();
        arr.push("There".to_string()).unwrap();
        arr.push("It is a beautiful day".to_string()).unwrap();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], "Hello");
        assert_eq!(arr[1], "There");
        assert_eq!(arr[2], "It is a beautiful day");

        assert_eq!(arr.remove(1).unwrap(), "There");

        assert_eq!(arr[0], "Hello");
        assert_eq!(arr[1], "It is a beautiful day");
    }

    #[test]
    fn clear() {
        let mut arr = FlexArr::<String, Global, u8>::new();
        assert!(arr.is_empty());

        arr.push("Hello".to_string()).unwrap();
        arr.push("There".to_string()).unwrap();
        arr.push("It is a beautiful day".to_string()).unwrap();
        let cap = arr.capacity();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], "Hello");
        assert_eq!(arr[1], "There");
        assert_eq!(arr[2], "It is a beautiful day");

        arr.clear();

        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.capacity(), cap);
    }

    #[test]
    fn truncate() {
        let mut arr = FlexArr::<String, Global, u8>::new();

        arr.push("Hello".to_string()).unwrap();
        arr.push("There".to_string()).unwrap();
        arr.push("It is a beautiful day".to_string()).unwrap();
        let cap = arr.capacity();

        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], "Hello");
        assert_eq!(arr[1], "There");
        assert_eq!(arr[2], "It is a beautiful day");

        arr.truncate(1);

        assert_eq!(arr.capacity(), cap);
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], "Hello");

        arr.truncate(2);
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn slice_method_ref() {
        let mut arr = FlexArr::<u8>::new();

        arr.push(0x2).unwrap();
        arr.push(0x3).unwrap();
        arr.push(0x6).unwrap();
        arr.push(0x0).unwrap();
        arr.push(0x5).unwrap();
        arr.push(0x4).unwrap();
        arr.push(0x1).unwrap();

        assert_eq!(*arr.last().unwrap(), 0x01);
    }

    #[test]
    fn slice_method_mut() {
        let mut arr = FlexArr::<u8>::new();

        arr.push(0x2).unwrap();
        arr.push(0x3).unwrap();
        arr.push(0x6).unwrap();
        arr.push(0x0).unwrap();
        arr.push(0x5).unwrap();
        arr.push(0x4).unwrap();
        arr.push(0x1).unwrap();

        arr.sort();

        for i in 0..arr.len() {
            assert_eq!(arr[i], i as u8);
        }
    }

    #[test]
    fn iter() {
        let mut arr = FlexArr::<u8>::new();

        arr.push(0x1).unwrap();
        arr.push(0x2).unwrap();
        arr.push(0x3).unwrap();
        arr.push(0x4).unwrap();
        arr.push(0x5).unwrap();
        arr.push(0x6).unwrap();
        arr.push(0x7).unwrap();

        let mut i = 1u8;
        for elem in &arr {
            assert_eq!(*elem, i);
            i += 1;
        }
        assert_eq!(i, 8);
    }

    #[test]
    fn iter_mut() {
        let mut arr = FlexArr::<u8>::new();

        arr.push(0x1).unwrap();
        arr.push(0x2).unwrap();
        arr.push(0x3).unwrap();
        arr.push(0x4).unwrap();
        arr.push(0x5).unwrap();
        arr.push(0x6).unwrap();
        arr.push(0x7).unwrap();

        let mut i = 1u8;
        for elem in &mut arr {
            assert_eq!(*elem, i);
            *elem += 1;
            i += 1;
        }
        assert_eq!(i, 8);

        i = 2;
        for elem in &arr {
            assert_eq!(*elem, i);
            i += 1;
        }
    }

    #[test]
    fn into_parts() {
        let mut arr = FlexArr::<String>::new();

        arr.push("Hello".to_string()).unwrap();
        arr.push("There".to_string()).unwrap();
        arr.push("It is a beautiful day".to_string()).unwrap();

        let (ptr, len, cap, alloc) = arr.into_parts();

        assert_eq!(len, 3);

        let str_ref = unsafe { ptr.as_ref().as_str() };
        assert_eq!(str_ref, "Hello");

        let str_ref = unsafe { ptr.add(1).as_ref().as_str() };
        assert_eq!(str_ref, "There");

        let str_ref = unsafe { ptr.add(2).as_ref().as_str() };
        assert_eq!(str_ref, "It is a beautiful day");

        // Make sure everything is dropped.
        let _ = unsafe { FlexArr::from_parts(ptr, len, cap, alloc) };
    }

    #[test]
    fn insert() {
        let mut arr = FlexArr::<u8>::new();
        arr.push(0x0).unwrap();
        arr.push(0x2).unwrap();
        arr.push(0x4).unwrap();

        assert_eq!(arr[0], 0x0);
        assert_eq!(arr[1], 0x2);
        assert_eq!(arr[2], 0x4);

        arr.insert(1, 0x1).unwrap();

        assert_eq!(arr[0], 0x0);
        assert_eq!(arr[1], 0x1);
        assert_eq!(arr[2], 0x2);
        assert_eq!(arr[3], 0x4);

        arr.insert(3, 0x3).unwrap();

        assert_eq!(arr[0], 0x0);
        assert_eq!(arr[1], 0x1);
        assert_eq!(arr[2], 0x2);
        assert_eq!(arr[3], 0x3);
        assert_eq!(arr[4], 0x4);

        arr.insert(5, 0x5).unwrap();

        assert_eq!(arr[0], 0x0);
        assert_eq!(arr[1], 0x1);
        assert_eq!(arr[2], 0x2);
        assert_eq!(arr[3], 0x3);
        assert_eq!(arr[4], 0x4);
        assert_eq!(arr[5], 0x5);

        let Err(ret) = arr.insert(7, 0x6) else {
            panic!("Insert should have failed!");
        };

        assert_eq!(ret.reason(), ErrorReason::IndexOutOfBounds);
    }
}
