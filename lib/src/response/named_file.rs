use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::ops::{Deref, DerefMut};

use response::{Response, Responder};
use http::{Status, ContentType};

/// A file with an associated name; responds with the Content-Type based on the
/// file extension.
#[derive(Debug)]
pub struct NamedFile(PathBuf, File);

impl NamedFile {
    /// Attempts to open a file in read-only mode.
    ///
    /// # Errors
    ///
    /// This function will return an error if path does not already exist. Other
    /// errors may also be returned according to
    /// [OpenOptions::open](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket::response::NamedFile;
    ///
    /// # #[allow(unused_variables)]
    /// let file = NamedFile::open("foo.txt");
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<NamedFile> {
        let file = File::open(path.as_ref())?;
        Ok(NamedFile(path.as_ref().to_path_buf(), file))
    }

    /// Retrieve the underlying `File`.
    #[inline(always)]
    pub fn file(&self) -> &File {
        &self.1
    }

    /// Take the underlying `File`.
    #[inline(always)]
    pub fn take_file(self) -> File {
        self.1
    }

    /// Retrieve a mutable borrow to the underlying `File`.
    #[inline(always)]
    pub fn file_mut(&mut self) -> &mut File {
        &mut self.1
    }

    /// Retrieve the path of this file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::io;
    /// use rocket::response::NamedFile;
    ///
    /// # #[allow(dead_code)]
    /// # fn demo_path() -> io::Result<()> {
    /// let file = NamedFile::open("foo.txt")?;
    /// assert_eq!(file.path().as_os_str(), "foo.txt");
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }
}

/// Streams the named file to the client. Sets or overrides the Content-Type in
/// the response according to the file's extension if the extension is
/// recognized. See
/// [ContentType::from_extension](/rocket/http/struct.ContentType.html#method.from_extension)
/// for more information. If you would like to stream a file with a different
/// Content-Type than that implied by its extension, use a `File` directly.
impl<'r> Responder<'r> for NamedFile {
    fn respond(self) -> Result<Response<'r>, Status> {
        let mut response = Response::new();
        if let Some(ext) = self.path().extension() {
            // TODO: Use Cow for lowercase.
            let ext_string = ext.to_string_lossy().to_lowercase();
            let content_type = ContentType::from_extension(&ext_string);
            if !content_type.is_any() {
                response.set_header(content_type);
            }
        }

        response.set_streamed_body(self.take_file());
        Ok(response)
    }
}

impl Deref for NamedFile {
    type Target = File;

    fn deref(&self) -> &File {
        &self.1
    }
}

impl DerefMut for NamedFile {
    fn deref_mut(&mut self) -> &mut File {
        &mut self.1
    }
}

impl io::Read for NamedFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file().read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.file().read_to_end(buf)
    }
}

impl io::Write for NamedFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file().flush()
    }
}

impl io::Seek for NamedFile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.file().seek(pos)
    }
}

impl<'a> io::Read for &'a NamedFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file().read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.file().read_to_end(buf)
    }
}

impl<'a> io::Write for &'a NamedFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file().flush()
    }
}

impl<'a> io::Seek for &'a NamedFile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.file().seek(pos)
    }
}
