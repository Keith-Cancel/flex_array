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
    pub const fn new_in(alloc: A) -> Self {
        return Self {
            inner: Inner::new_in(alloc, align_of::<T>()),
            len:   L::ZERO_VALUE,
            _ph:   PhantomData,
        };
    }

    pub fn with_capacity(alloc: A, capacity: L) -> FlexArrResult<Self> {
        let lay = Layout::new::<T>();
        let mut inner = Inner::new_in(alloc, lay.align());
        inner.expand_by(capacity, lay)?;
        return Ok(Self {
            inner: inner,
            len:   L::ZERO_VALUE,
            _ph:   PhantomData,
        });
    }

    pub const fn capacity(&self) -> L {
        return self.inner.capacity(size_of::<T>());
    }

    pub const fn len(&self) -> L {
        return self.len;
    }
}
