//! Contains types that are use by `FlexArr` The important types are the `LengthType`
//! and `FlexArrErr`.
//!
//! `LengthType` is a trait that is used for letting you specify the type of the length,
//! capacity, and indexing operations.
//!
//! `FlexArrErr` is type that that used to indicate an error during a `FlexArr` operation.
//!
//! `FlexArrResult` is a type alias for `Result<T, FlexArrErr>`
mod errors;
mod len_type;

pub use errors::*;
pub use len_type::LengthType;
