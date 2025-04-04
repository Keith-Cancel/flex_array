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
//! - `alloc_unstable` – Enables support for Rust’s unstable `Allocator` trait for custom
//!   allocators. When used with `std_alloc`, this re-exports the `Global` type from `std` instead
//!   of the custom wrapper named `Global`.
//!
//! - `alloc_api2`
//!   Enables support for the `allocator-api2` crate, which also provides an `Allocator` trait
//!   in stable rust. This way if you already have allocators written against this you can use
//!   them with the `flex_array` crate.
//!   **Note:** This feature should not be enabled with `alloc_unstable` feature. If
//!   you want to use both just able to enable `alloc_unstable` and `nightly` in the
//!   `allocator-api2` crate. Additionally, if you are using the `nightly` feature of the
//!  `allocator-api2` crate you will need to enable the `alloc_unstable` feature.

#![no_std]
#![cfg_attr(feature = "alloc_unstable", feature(allocator_api))]

#[cfg(any(feature = "std_alloc", test))]
extern crate std;

pub mod alloc;
mod flex_array;
pub mod types;

pub use flex_array::FlexArr;

// Kinda annoying I could avoid this with specialization, but I can only have one blanket impl for AltAllocator unless
// I used specialization. However, I decided against having a specialization flag. Specialization has soundness holes
// and can crash the compiler. So just treat this as a compiler error for now. =(
#[cfg(all(feature = "alloc_api2", feature = "alloc_unstable"))]
compile_error!(
    "Cannot enable both `alloc_api2` and `alloc_unstable` features, Instead, enable `nightly` for the Allocator Api2 crate and just `alloc_unstable`"
);
