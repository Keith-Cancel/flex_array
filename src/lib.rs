//! # Flexible Array
//!
//! The `flex_array` crate provides a `#[no_std]` flexible array much like  `std::Vec but` with the ability
//! to customize the type used for the length, capacity, and indexing operations.
//! I wrote `FlexArr` to address some of the limitations of Rust’s standard `Vec`.
//!
//! `FlexArr` uses fallible allocations, meaning that instead of panicking on allocation failure,
//! it returns an error. This allow one to handle the error in a more graceful or robust manner.
//! `Vec` does have some fallible allocation methods, but most are currently unstable.
//!
//! In addition, one can customize the type used for the length, capacity, and indexing operations.
//! For example on a 64-bit system, the standard `Vec` typically uses 24 bytes. `FlexArr` specifying
//! a smaller type than `usize` as a generic (e.g. `u32`) with `FlexArr` can reduce this overhead to
//! just 16 bytes.
//!
//! Lastly, the allocator API is not stable yet, so this crate provides and alternate trait `AltAllocator`
//! that works like `Allocator` the trait and can be used with `FlexArr`
//!
//! # Feature Flags
//! * `std_alloc` - This feature enables a wrapper called `Global` that implements that implements
//! `AltAllocator` using the standard allocator APIs.
//!
//! * `experimental_allocator` - This feature enables the use of the unstable `Allocator` trait for
//! custom memory allocators. Further, if used in conjunction with `std_alloc` this will re-export
//! the `Global` type from the `std` crate instead of the `Global` wrapper defined in this crate.

#![no_std]
#![cfg_attr(feature = "experimental_allocator", feature(allocator_api))]

#[cfg(any(feature = "std_alloc", test))]
extern crate std;

mod flex_array;
pub mod types;

pub use flex_array::FlexArr;
