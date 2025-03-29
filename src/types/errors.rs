#[cfg(feature = "experimental_allocator")]
pub use core::alloc::AllocError;
use core::error::Error;
use core::fmt;

/// This indicates some sort of memory allocation error for the alt allocator.
///
/// If the rust allocator API is enabled this will be the same error type as
/// the Allocator API.
#[cfg(not(feature = "experimental_allocator"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AllocError;

#[cfg(not(feature = "experimental_allocator"))]
impl Error for AllocError {}

#[cfg(not(feature = "experimental_allocator"))]
impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("A memory allocation error occurred.")
    }
}

/// This enum lets one figure out what kind of error occurred durning
/// a `FlexArr` operation.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    CapacityOverflow = 1,
    UsizeOverflow,
    LayoutFailure,
    AllocFailure,
}

/// A type alias for `Result<T, FlexArrErr>`
pub type FlexArrResult<T> = Result<T, FlexArrErr>;

/// This is used to indicate an error during a `FlexArr` operation.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FlexArrErr(ErrorKind);

impl FlexArrErr {
    pub(crate) const fn new(kind: ErrorKind) -> Self {
        return Self(kind);
    }
    pub const fn kinda(self) -> ErrorKind {
        return self.0;
    }
}

impl Error for FlexArrErr {}

impl fmt::Display for FlexArrErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ErrorKind::CapacityOverflow => f.write_str("Capacity type overflowed."),
            ErrorKind::UsizeOverflow => f.write_str("usize overflowed."),
            ErrorKind::LayoutFailure => f.write_str("Failed to create layout."),
            ErrorKind::AllocFailure => f.write_str("An allocation failure occurred."),
        }
    }
}
