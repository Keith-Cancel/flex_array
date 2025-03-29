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
