#[cfg(all(feature = "read", feature = "write"))]
pub mod objcopy;

#[cfg(feature = "read")]
pub mod objdump;

#[cfg(feature = "read")]
pub mod readobj;
