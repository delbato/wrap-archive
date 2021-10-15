use std::io::{
    Result as IoResult,
    Seek,
    SeekFrom,
    Write,
};

use blake3::Hasher;

/// HashWriter struct
///
/// Wraps another writer and recors the written bytes to
/// a BLAKE2S hasher, whose hash you can retrieve later
pub struct HashWriter<'w, W: Write + Seek + 'w> {
    /// A reference to the underlying writer
    writer: &'w mut W,
    /// The BLAKE3 hasher
    hasher: Hasher,
}

impl<'w, W: Write + Seek + 'w> HashWriter<'w, W> {
    /// Creates a new instance, wrapping the given writer
    pub fn new(writer: &'w mut W) -> Self {
        Self {
            writer: writer,
            hasher: Hasher::new(),
        }
    }

    /// Retrieves the BLAKE2S hash as a 32B array, resetting the hasher
    pub fn get_hash(&mut self) -> [u8; 32] {
        *self.hasher.finalize().as_bytes()
    }
}

impl<'w, W: Write + Seek + 'w> Write for HashWriter<'w, W> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let written = self.writer.write(buf)?;
        self.hasher.update(&buf[0..written]);
        Ok(written)
    }
    fn flush(&mut self) -> IoResult<()> {
        self.writer.flush()
    }
}

impl<'w, W: Write + Seek + 'w> Seek for HashWriter<'w, W> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        self.writer.seek(pos)
    }
}
