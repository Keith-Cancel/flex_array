use super::inner::Inner;
use crate::types::AltAllocator;
use crate::types::LengthType;
use core::marker::PhantomData;

pub struct FlexArr<T, L: LengthType, A: AltAllocator>
where
    usize: TryFrom<L>,
{
    inner: Inner<L, A>,
    len: L,
    _ph: PhantomData<T>,
}
