#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "aes")]
extern crate aes_ctr;
extern crate bincode;
pub extern crate blake3;

/// Read functionality module
pub mod read;

/// Write functionality module
pub mod write;

/// Shared functionality module
pub mod shared;

pub use read::archive::Archive;
pub use read::file::File;
pub use shared::compression::Compression;
pub use write::writer::Writer;

/// File format version
pub const FILE_FORMAT_VERSION: u8 = 1;