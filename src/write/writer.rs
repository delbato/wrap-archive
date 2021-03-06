use std::io::{
    Seek,
    SeekFrom,
    Write,
};

use bincode::serialize_into as bincode_write;

use crate::shared::archive_header::ArchiveHeader;
use crate::shared::compression::Compression;
use crate::shared::directory::Directory;
use crate::shared::encryption::Encryption;
use crate::shared::error::{
    Error,
    Result,
};
use crate::write::file::File;
/// The Writer struct
///
/// Used for creating .var Archives
pub struct Writer<W: Write + Seek> {
    /// The internal, root-level writer
    sink: W,
    /// The directory struct, mapping filenames to byte ranges
    directory: Directory,
}

/// Creates a new Writer, wrapping a given Write struct
impl<'w, W: Write + Seek> Writer<W> {
    /// Creates a new instance, wrapping the given writer
    pub fn new(mut sink: W) -> Result<Writer<W>> {
        sink.write(b"WRAP").map_err(|_| Error::CantWriteMagicBytes)?;
        let archive_header = ArchiveHeader::default();
        bincode_write(&mut sink, &archive_header).map_err(|_| Error::CantWriteHeader)?;
        Ok(Self {
            sink: sink,
            directory: Directory::default(),
        })
    }

    /// Initiates a new file writer at the given path
    pub fn write_file(
        &'w mut self,
        path: &str,
        compression_type: Compression,
    ) -> Result<File<'w, W>> {
        File::new(
            &mut self.sink,
            &mut self.directory,
            path,
            compression_type,
            Encryption::default(),
        )
    }

    /// Finishes and drops the archive writer
    pub fn finish(self) {}
}

impl<'w, W: Write + Seek> Drop for Writer<W> {
    fn drop(&mut self) {
        let directory_begin = self.sink.seek(SeekFrom::End(0)).unwrap();
        bincode_write(&mut self.sink, &self.directory).unwrap();
        let directory_end = self.sink.seek(SeekFrom::Current(0)).unwrap();
        self.sink.seek(SeekFrom::Start(4)).unwrap();
        let mut archive_header = ArchiveHeader::default();
        archive_header.directory_range = directory_begin..directory_end;
        bincode_write(&mut self.sink, &archive_header).unwrap();
    }
}
