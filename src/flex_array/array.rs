use core::alloc::Layout;
use core::marker::PhantomData;
use core::ops::Index;
use core::ops::IndexMut;
use core::ptr;
use core::slice;

use super::inner::Inner;
use crate::types::AltAllocator;
use crate::types::ErrorCause;
use crate::types::FlexArrErr;
use crate::types::FlexArrResult;
use crate::types::LengthType;

#[derive(Debug)]
pub struct FlexArr<T, A: AltAllocator, L: LengthType = u32>
where
    usize: TryFrom<L>,
{
    inner: Inner<A, L>,
    _ph:   PhantomData<T>,
}

impl<T, A: AltAllocator, L: LengthType> FlexArr<T, A, L>
where
    usize: TryFrom<L>,
{
    const LAYOUT: Layout = Layout::new::<T>();
    const SIZE: usize = size_of::<T>();

    pub const fn new_in(alloc: A) -> Self {
        return Self {
            inner: Inner::new_in::<T>(alloc),
            _ph:   PhantomData,
        };
    }

    pub fn with_capacity(alloc: A, capacity: L) -> FlexArrResult<Self> {
        let mut inner = Inner::new_in::<T>(alloc);
        inner.expand_by(capacity, Self::LAYOUT)?;
        return Ok(Self {
            inner: inner,
            _ph:   PhantomData,
        });
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.inner.length;
        if len == L::ZERO_VALUE {
            return None;
        }
        let ret = unsafe { ptr::read(self.as_ptr().add(len.as_usize())) };
        self.inner.length = len - L::ONE_VALUE;
        return Some(ret);
    }

    pub fn push(&mut self, item: T) -> FlexArrResult<()> {
        let len = self.inner.length;

        if len >= self.capacity() {
            self.inner.expand_at_least_by(L::ONE_VALUE, Self::LAYOUT)?;
        }

        let Ok(len) = usize::try_from(len) else {
            return Err(FlexArrErr::new(ErrorCause::UsizeOverflow));
        };

        let loc = unsafe { self.as_mut_ptr().add(len) };
        unsafe { ptr::write(loc, item) };
        self.inner.length += L::ONE_VALUE;

        return Ok(());
    }

    pub const fn capacity(&self) -> L {
        return self.inner.capacity(Self::SIZE);
    }

    #[inline]
    pub const fn len(&self) -> L {
        return self.inner.length;
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.inner.length.as_usize()) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.inner.length.as_usize()) }
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

impl<T, A: AltAllocator, L: LengthType> Index<L> for FlexArr<T, A, L>
where
    usize: TryFrom<L>,
{
    type Output = T;
    fn index(&self, index: L) -> &Self::Output {
        let i = index.as_usize();
        return &self.as_slice()[i];
    }
}

impl<T, A: AltAllocator, L: LengthType> IndexMut<L> for FlexArr<T, A, L>
where
    usize: TryFrom<L>,
{
    fn index_mut(&mut self, index: L) -> &mut Self::Output {
        let i = index.as_usize();
        return &mut self.as_mut_slice()[i];
    }
}
