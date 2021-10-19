use std::io::{
    Read,
    Seek,
    SeekFrom,
};
use std::fs::File as StdFile;
use std::ops::Range;
use std::path::Path;

use bincode::deserialize_from as bincode_read;

use crate::read::file::File;
use crate::shared::archive_header::ArchiveHeader;
use crate::shared::directory::Directory;
use crate::shared::error::{
    Error,
    Result,
};
use crate::shared::file_header::FileHeader;
use crate::shared::file_info::FileInfo;

/// The Archive struct
pub struct Archive<R: Read + Seek> {
    /// The source - e.g. a file
    source: R,
    /// The directory
    directory: Directory,
}

impl Archive<StdFile> {
    /// Opens an archive file at a given Path
    pub fn open<'p, P: Into<&'p Path>>(path: P) -> Result<Self> {
        let path = path.into();
        let file = StdFile::open(path)
            .map_err(|_| Error::Unknown)?;
        Self::new(file)
    }
}

impl<'r, R: Read + Seek + 'r> Archive<R> {
    /// Creates a new archive, wrapping the given source
    pub fn new(mut source: R) -> Result<Self> {
        // Read the magic bytes
        let mut magic_bytes = [0u8; 4];
        source
            .read_exact(&mut magic_bytes)
            .map_err(|_| Error::CantReadMagicBytes)?;
        if &magic_bytes != b"WRAP" {
            return Err(Error::IncorrectMagicBytes(magic_bytes));
        }
        let header: ArchiveHeader = bincode_read(&mut source)
            .map_err(|_| Error::CorruptHeader)?;
        let dir_begin = header.directory_range.start;
        source
            .seek(SeekFrom::Start(dir_begin))
            .map_err(|_| Error::CantReadDirectory)?;
        let directory = bincode_read(&mut source)
            .map_err(|_| Error::CorruptDirectory)?;
        Ok(Self {
            directory: directory,
            source: source,
        })
    }

    /// Lists all files
    pub fn get_file_list(&self) -> Vec<String> {
        self.directory.get_file_list()
    }

    /// Gets a File stream by path
    pub fn get_file(&'r mut self, path: &str) -> Result<File<'r, R>> {
        let file_header_range = self
            .directory
            .get_file(path)
            .ok_or(Error::FileNotFound(String::from(path)))?;
        let file_header = self.get_file_header(file_header_range)?;
        File::new(
            &mut self.source,
            file_header.file_info.raw_size,
            file_header.data_range.clone(),
            file_header.file_info.compression.clone(),
        )
    }

    /// Gets a files info by path
    pub fn get_file_info(&mut self, path: &str) -> Result<FileInfo> {
        let file_header_range = self
            .directory
            .get_file(path)
            .ok_or(Error::FileNotFound(String::from(path)))?;
        let file_header = self.get_file_header(file_header_range)?;
        let file_info = file_header.file_info.clone().with_filename(path);
        Ok(file_info)
    }

    /// Gets a file header
    fn get_file_header(&mut self, file_header_range: Range<u64>) -> Result<FileHeader> {
        let file_header_start = file_header_range.start;
        let file_header_range = file_header_range.start as usize..file_header_range.end as usize;
        let file_header_size = file_header_range.end - file_header_range.start;
        let mut file_header_bytes = vec![];
        file_header_bytes.resize(file_header_size, 0);
        self.source
            .seek(SeekFrom::Start(file_header_start))
            .map_err(|_| Error::CantReadFileHeader)?;
        self.source
            .read_exact(&mut file_header_bytes)
            .map_err(|_| Error::CantReadFileHeader)?;
        bincode_read(&*file_header_bytes).map_err(|_| Error::CorruptFileHeader)
    }
}
