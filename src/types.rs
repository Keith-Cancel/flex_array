//! Contains mainly traits used by `FlexArr` The important types are the `LengthType`
//! trait and the `AltAllocator` trait.
//!
//! If the `experimental_allocator` feature is enabled, the `AltAllocator`
//! trait is implemented for types that implement the allocator api `Allocator` trait.
//!
//! If built with the `std_alloc` feature, a wrapper called `Global` is also
//! provided. Further, if the `experimental_allocator` feature is enabled,
//! the allocator APIs `Global` is re-exported instead.
mod errors;
mod len_type;

pub use errors::*;
pub use len_type::LengthType;
