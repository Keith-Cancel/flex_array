# Flexible Array

The `flex_array` crate provides a `#[no_std]` flexible array similar to `std::Vec`, but with enhanced control over memory usage and error handling. By allowing you to customize the types used for length, capacity, and indexing operations, `FlexArr` an alternative to Rust’s `std::Vec` and you can use it in a lot places where you might otherwise use `Vec`.

## Why Use `FlexArr`?

I created `FlexArr` to address some of the limitations inherent in Rust’s standard `Vec`. Here are the key benefits:

- **Reduced Memory Overhead**
  On a 64-bit system, a typical `std::Vec` uses 24 bytes for its metadata. By choosing a smaller integer type (e.g., `u32`) for length and capacity, `FlexArr` can reduce this overhead to just 16 bytes, making it especially useful for to help reduce memory bloat or in embedded systems with limited RAM.

- **Fallible Allocations**
  Unlike `std::Vec`, which may panic on allocation failure, `FlexArr` employs fallible allocations. Instead of crashing your application, it returns a `FlexArrErr`, allowing you to implement more graceful error handling. This is particularly advantageous in environments where stability is critical.

- **Custom Allocator Support**
  Rust’s standard allocator API remains unstable. To work around this, `FlexArr` introduces the `AltAllocator` trait as an alternative to the standard `Allocator` trait. With `FlexArr`, you can easily integrate custom memory allocators, and if the allocator API stabilizes in the future, `AltAllocator` can effectively become an alias for `Allocator`.

## Key Features

- **Customizable Metadata Types**
  Tailor the size of your dynamic array’s metadata by choosing a custom `LengthType`. This allows you to balance memory usage and performance based on your application's needs.

- **`no_std` Compatibility**
  Designed to work in environments where the Rust standard library is not available, making it ideal for embedded systems and other constrained platforms.

## Feature Flags

- **`std_alloc`**
  Enables a wrapper type called `Global` that implements `AltAllocator` using the standard allocator APIs. This provides a drop-in replacement for applications that rely on the standard memory allocator.

- **`experimental_allocator`**
  Enables support for Rust’s unstable `Allocator` trait for custom memory allocators. When used in conjunction with `std_alloc`, this feature re-exports the `Global` type directly from the `std` crate rather than using the custom `Global` wrapper provided by `flex_array`.

## Getting Started

Add `flex_array` to your `Cargo.toml`. If you want to enable the std allocator enable the `std_alloc` feature.
```toml
[dependencies]
flex_array = "0.1.0"
```

```rust
use flex_array::FlexArr;

fn main() {
    // FlexArr with u32 length/capacity and if using the std allocator.
    let mut array: FlexArr<i32> = FlexArr::new();
    // Or
    // Create an empty FlexArr with a custom allocator and a u16 for length/capacity.
    let mut array: FlexArr<i32, YourAllocator, u16> = FlexArr::new_in(your_allocator);

    // Reserve capacity for 100 elements.
    array.reserve(100).expect("Failed to allocate memory");

    // Push elements into the array.
    array.push(42).expect("Failed to push element");

    // Access elements safely.
    if let Some(value) = array.get(0) {
        println!("First element: {}", value);
    }
}
```

## Feedback and Contributions

I welcome any feedback and contributions! If you have suggestions for improvements, encounter any bugs, or discover potential soundness issues, please open an issue or submit a pull request. Although I've run tests and used Miri to check for issues. The unsafe code required to get a `Vec` like container while supporting multiple allocators means if you find any soundness issues please let me know so it can be dealt with promptly. Any input is kindly appreciated to ensure the quality and reliability of this crate.

## License
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT))
