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
    Self: cmp::PartialEq,
    Self: cmp::PartialOrd,
    Self: ops::Add<Output = Self>,
    Self: ops::AddAssign,
    Self: ops::Mul<Output = Self>,
    Self: ops::MulAssign,
    Self: ops::Shr<Output = Self>,
    Self: ops::ShrAssign,
    Self: ops::Sub<Output = Self>,
    Self: ops::SubAssign,
    Self: Sized,
    Self: From<u8>,
    usize: TryFrom<Self>,
{
    const MIN_VALUE: Self;
    const MAX_VALUE: Self;
    const ONE_VALUE: Self;
    const ZERO_VALUE: Self;

    fn as_usize(self) -> usize;
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
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
            #[inline]
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                return self.checked_mul(rhs);
            }
        }
    };
}

impl_length_type!(usize);
impl_length_type!(u8);
impl_length_type!(u16);
impl_length_type!(u32);
impl_length_type!(u64);
