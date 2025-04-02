//! # Flexible Array
//!
//! The `flex_array` crate provides a `#[no_std]` flexible array similar to `std::Vec`, but with
//! greater control over memory usage. It allows customization of the types used for length,
//! capacity, and indexing operations, making it more efficient in certain use cases.
//!
//! ## Why Use `FlexArr`?
//!
//! I wrote `FlexArr` to address some limitations of Rust’s standard `std::Vec`:
//!
//! - **Reduced Memory Overhead**: The `std::Vec` on a 64-bit system typically uses 24 bytes.
//!   By specifying a smaller type for length and capacity (e.g., `u32`), `FlexArr` can reduce
//!   this to just 16 bytes, helping one optimizing memory usage.
//!
//! - **Fallible Allocations**: Instead of panicking on allocation failure, `FlexArr` returns an
//!   error, allowing for more robust or graceful error handling. While `std::Vec` has some fallible
//!   methods, most are still unstable.
//!
//! - **Custom Allocator Support**: Since Rust’s allocator API is unstable, `FlexArr` introduces
//!   the `AltAllocator` trait as an alternative to the standard `Allocator` trait. If the the
//!   allocator API is stabilized, this will basically just become an an alias for `Allocator`.
//!
//! ## Feature Flags
//!
//! - `std_alloc` – Enables a wrapper called `Global` that implements `AltAllocator` using the standard
//!   allocator APIs.
//!
//! - `experimental_allocator` – Enables support for Rust’s unstable `Allocator` trait for custom
//!   allocators. When used with `std_alloc`, this re-exports the `Global` type from `std` instead
//!   of the custom wrapper named `Global`.
#![no_std]
#![cfg_attr(feature = "experimental_allocator", feature(allocator_api))]

#[cfg(any(feature = "std_alloc", test))]
extern crate std;

pub mod alloc;
mod flex_array;
pub mod types;

pub use flex_array::FlexArr;
