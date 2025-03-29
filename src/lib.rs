#![no_std]
#![cfg_attr(feature = "experimental_allocator", feature(allocator_api))]

#[cfg(any(feature = "std_alloc", test))]
extern crate std;

pub mod flex_array;
pub mod types;
