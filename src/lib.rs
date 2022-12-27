//! Lightweight library for handling Android Boot Images (including Samsung!)
#![deny(
    warnings,
    missing_copy_implementations,
    unused_imports,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

extern crate byteorder;
#[macro_use]
extern crate quick_error;

mod header;
mod image;

pub use header::samsung_header::SAMSUNG_HEADER_SIZE;
pub use header::SamsungHeader;
pub use image::samsung::{BadHeaderError, BootImage, ReadBootImageError};
