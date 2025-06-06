//! Contains mainly allocator types and traits used by `FlexArr` The most important being
//! the `AltAllocator` trait, and the `AllocError` type.
//!
//! If the `alloc_unstable` feature is enabled, the `AltAllocator`
//! trait is implemented for types that implement the allocator api `Allocator` trait.
//!
//! If built with the `std_alloc` feature, a wrapper called `Global` is also
//! provided. Further, if the `alloc_unstable` feature is enabled,
//! the allocator APIs `Global` is re-exported instead.

#[cfg(all(feature = "alloc_api2", not(feature = "alloc_unstable")))]
mod alloc_api2;
#[cfg(feature = "alloc_unstable")]
mod alloc_unstable;
mod alt_alloc;
#[cfg(feature = "std_alloc")]
mod std_alloc;

#[cfg(feature = "alloc_unstable")]
pub use core::alloc::AllocError;

#[cfg(not(feature = "alloc_unstable"))]
pub use alloc_error::AllocError;
pub use alt_alloc::AltAllocator;
#[cfg(feature = "std_alloc")]
pub use std_alloc::Global;

#[cfg(not(feature = "alloc_unstable"))]
mod alloc_error {
    use core::error::Error;
    use core::fmt;

    /// This indicates some sort of memory allocation error for the alt allocator.
    ///
    /// If the rust allocator API is enabled this will be the same error type as
    /// the Allocator API.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct AllocError;

    impl Error for AllocError {}

    impl fmt::Display for AllocError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("A memory allocation error occurred.")
        }
    }
}
