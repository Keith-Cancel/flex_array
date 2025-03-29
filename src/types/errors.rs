#[cfg(feature = "experimental_allocator")]
pub use core::alloc::AllocError;
#[cfg(feature = "experimental_allocator")]
use core::alloc::Allocator;

/// This indicates some sort of memory allocation error for the alt allocator.
///
/// If the rust allocator API is enabled this will be the same error type as
/// the Allocator API.
#[cfg(not(feature = "experimental_allocator"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AllocError;

#[cfg(not(feature = "experimental_allocator"))]
impl core::error::Error for AllocError {}

#[cfg(not(feature = "experimental_allocator"))]
impl core::fmt::Display for AllocError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("A memory allocation error occurred.")
    }
}
