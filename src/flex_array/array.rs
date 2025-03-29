use core::alloc::Layout;
use core::marker::PhantomData;
use core::mem::align_of;

use super::inner::Inner;
use crate::types::AltAllocator;
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
    const ALIGN: usize = align_of::<T>();

    pub const fn new_in(alloc: A) -> Self {
        return Self {
            inner: Inner::new_in(alloc, Self::ALIGN),
            len:   L::ZERO_VALUE,
            _ph:   PhantomData,
        };
    }

    pub fn with_capacity(alloc: A, capacity: L) -> FlexArrResult<Self> {
        let mut inner = Inner::new_in(alloc, Self::ALIGN);
        inner.expand_by(capacity, Self::LAYOUT)?;
        return Ok(Self {
            inner: inner,
            len:   L::ZERO_VALUE,
            _ph:   PhantomData,
        });
    }

    /*
    pub fn push(&mut self, item: T) -> FlexArrResult<()> {
        if self.len >= self.inner.real_capacity() {
            self.inner.expand_at_least_by(L::from(1u8), Self::LAYOUT)?;
        }

        return Ok(());
    }*/

    pub const fn capacity(&self) -> L {
        return self.inner.capacity(Self::SIZE);
    }

    #[inline]
    pub const fn len(&self) -> L {
        return self.len;
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
