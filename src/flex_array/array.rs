use core::alloc::Layout;
use core::marker::PhantomData;
use core::ptr;
use core::slice;

use super::inner::Inner;
use crate::types::AltAllocator;
use crate::types::ErrorKind;
use crate::types::FlexArrErr;
use crate::types::FlexArrResult;
use crate::types::LengthType;

pub struct FlexArr<T, L: LengthType, A: AltAllocator>
where
    usize: TryFrom<L>,
{
    inner: Inner<L, A>,
    len:   L,
    _ph:   PhantomData<T>,
}

impl<T, L: LengthType, A: AltAllocator> FlexArr<T, L, A>
where
    usize: TryFrom<L>,
{
    const LAYOUT: Layout = Layout::new::<T>();
    const SIZE: usize = size_of::<T>();

    pub const fn new_in(alloc: A) -> Self {
        return Self {
            inner: Inner::new_in::<T>(alloc),
            len:   L::ZERO_VALUE,
            _ph:   PhantomData,
        };
    }

    pub fn with_capacity(alloc: A, capacity: L) -> FlexArrResult<Self> {
        let mut inner = Inner::new_in::<T>(alloc);
        inner.expand_by(capacity, Self::LAYOUT)?;
        return Ok(Self {
            inner: inner,
            len:   L::ZERO_VALUE,
            _ph:   PhantomData,
        });
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == L::ZERO_VALUE {
            return None;
        }
        let ret = unsafe { ptr::read(self.as_ptr().add(self.len.as_usize())) };
        self.len -= L::ONE_VALUE;
        return Some(ret);
    }

    pub fn push(&mut self, item: T) -> FlexArrResult<()> {
        let len = self.len;

        if len >= self.capacity() {
            self.inner.expand_at_least_by(L::ONE_VALUE, Self::LAYOUT)?;
        }

        let Ok(len) = usize::try_from(len) else {
            return Err(FlexArrErr::new(ErrorKind::UsizeOverflow));
        };

        let loc = unsafe { self.as_mut_ptr().add(len) };
        unsafe { ptr::write(loc, item) };
        self.len += L::ONE_VALUE;

        return Ok(());
    }

    pub const fn capacity(&self) -> L {
        return self.inner.capacity(Self::SIZE);
    }

    #[inline]
    pub const fn len(&self) -> L {
        return self.len;
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len.as_usize()) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len.as_usize()) }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        return self.inner.get_ptr();
    }

    #[inline]
    pub const fn as_mut_ptr(&self) -> *mut T {
        return self.inner.get_ptr();
    }
}
