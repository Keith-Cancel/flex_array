//! Contains mainly traits used by `FlexArr` The important types are the `LengthType`
//! trait and the `AltAllocator` trait.
//!
//! If the `experimental_allocator` feature is enabled, the `AltAllocator`
//! trait is implemented for types that implement the allocator api `Allocator` trait.
//!
//! If built with the `std_alloc` feature, a wrapper called `Global` is also
//! provided. Further, if the `experimental_allocator` feature is enabled,
//! the allocator APIs `Global` is re-exported instead.
mod alt_alloc;
mod errors;
mod len_type;
#[cfg(feature = "std_alloc")]
mod std_alloc;

pub use alt_alloc::AltAllocator;
pub use errors::*;
pub use len_type::LengthType;
#[cfg(feature = "std_alloc")]
pub use std_alloc::Global;
