mod alt_alloc;
#[cfg(feature = "std_alloc")]
mod std_alloc;

#[cfg(feature = "experimental_allocator")]
pub use core::alloc::AllocError;

pub use alt_alloc::AltAllocator;
#[cfg(feature = "std_alloc")]
pub use std_alloc::Global;

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
