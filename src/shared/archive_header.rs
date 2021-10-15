use std::ops::Range;

use serde::{
    Deserialize,
    Serialize,
};
/// The archive header
///
/// Contains archive metadata and points to the most up-to-date directory.
#[derive(Serialize, Deserialize, Debug)]
pub struct ArchiveHeader {
    /// The library version used to create the archive
    ///
    /// Format: 16B long array with a UTF-8 encoded version string,
    /// preferably using semantic versioning
    pub version: u8,
    /// Byte range that points to the most up-to-date directory
    pub directory_range: Range<u64>,
}

impl Default for ArchiveHeader {
    fn default() -> Self {
        Self {
            version: crate::FILE_FORMAT_VERSION,
            directory_range: 0..0,
        }
    }
}
