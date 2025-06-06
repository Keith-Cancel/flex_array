use core::error::Error;
use core::fmt;

/// This enum lets one figure out what the reason an error occurred durning
/// a `FlexArr` operation.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorReason {
    CapacityOverflow = 1,
    UsizeOverflow,
    LayoutFailure,
    AllocFailure,
    IndexOutOfBounds,
}

/// A type alias for `Result<T, FlexArrErr>`
pub type FlexArrResult<T> = Result<T, FlexArrErr>;

/// This is used to indicate an error during a `FlexArr` operation.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FlexArrErr(ErrorReason);

impl FlexArrErr {
    pub(crate) const fn new(reason: ErrorReason) -> Self {
        return Self(reason);
    }
    pub const fn reason(self) -> ErrorReason {
        return self.0;
    }
}

impl Error for FlexArrErr {}

impl fmt::Display for FlexArrErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ErrorReason::CapacityOverflow => f.write_str("Capacity type overflowed."),
            ErrorReason::UsizeOverflow => f.write_str("usize overflowed."),
            ErrorReason::LayoutFailure => f.write_str("Failed to create layout."),
            ErrorReason::AllocFailure => f.write_str("An allocation failure occurred."),
            ErrorReason::IndexOutOfBounds => f.write_str("A given index is out of bounds."),
        }
    }
}
