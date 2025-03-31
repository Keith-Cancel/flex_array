use core::cmp;
use core::ops;

/// This trait is used for letting you specify the type of the length and
/// capacity fields of a flex array container, As well as indexing operations.
/// If you have some other type that you want to use and behaves like an unsigned integer,
/// you can implement this trait for it.
///
/// It's marked as unsafe since your type must be continuous and ordered.
/// under common operations such as addition multiplication like integers.
pub unsafe trait LengthType
where
    Self: Copy,
    Self: cmp::Eq,
    Self: cmp::Ord,
    Self: ops::Add<Output = Self>,
    Self: ops::Shr<Output = Self>,
    Self: ops::Sub<Output = Self>,
    Self: Sized,
    Self: From<u8>,
    Self: TryFrom<usize>,
    usize: TryFrom<Self>,
{
    /// The minimum value for this type.
    const MIN_VALUE: Self;
    /// The maximum value for this type.
    const MAX_VALUE: Self;
    /// The representation of `1` for this type.
    const ONE_VALUE: Self;
    /// The representation of `0` for this type.
    const ZERO_VALUE: Self;

    /// Converts this type to a `usize`. This will only
    /// be called when the value by `FlexArr`` if the same
    /// value at some point successfully used `usize::try_from(self)`.
    ///
    /// An implementation could be simple as:
    /// ```Self as usize```
    fn as_usize(self) -> usize;
    /// The same as `checked_add` for rust's built in types.
    fn checked_add(self, rhs: Self) -> Option<Self>;
    /// The same as `checked_sub` for rust's built in types.
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    /// The same as `wrapping_add` for rust's built in types.
    fn wrapping_add(self, rhs: Self) -> Self;
}

macro_rules! impl_length_type {
    ($typ:ty) => {
        unsafe impl LengthType for $typ {
            const MIN_VALUE: Self = Self::MIN;
            const MAX_VALUE: Self = Self::MAX;
            const ONE_VALUE: Self = 1;
            const ZERO_VALUE: Self = 0;

            #[inline(always)]
            fn as_usize(self) -> usize {
                return self as usize;
            }

            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                return self.checked_add(rhs);
            }

            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                return self.checked_sub(rhs);
            }

            #[inline(always)]
            fn wrapping_add(self, rhs: Self) -> Self {
                return self.wrapping_add(rhs);
            }
        }
    };
}

impl_length_type!(usize);
impl_length_type!(u8);
impl_length_type!(u16);
impl_length_type!(u32);
impl_length_type!(u64);
impl_length_type!(u128);
