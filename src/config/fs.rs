//! A small trait over the file system, so tests give fakes (decision 17).

use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt};
use std::path::Path;

/// The start of the name of a temporary copy of the key file. The name goes on
/// with the process id and a random part (`write_key_file`).
pub const TEMP_PREFIX: &str = "config.yml.tmp-";

/// An open file. The mode comes from the open handle (`fstat`), so no other
/// file can take its place between the check and the read.
pub trait OpenFile {
    fn mode(&self) -> io::Result<u32>;
    fn read_to_string(&mut self) -> io::Result<String>;
}

pub trait Fs {
    /// Open a file for reading. This follows a symbolic link (decision 20).
    fn open(&self, path: &Path) -> io::Result<Box<dyn OpenFile>>;
    /// True when the path is a symbolic link. A missing path is not a link.
    fn is_symlink(&self, path: &Path) -> io::Result<bool>;
    /// True when the path exists and is not a regular file, after jag follows
    /// each link (a folder or a pipe, for example). A missing path is not special.
    fn is_special(&self, path: &Path) -> io::Result<bool>;
    /// Create a folder and its parents with the default mode.
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    /// Create one folder with mode `0o700`. A folder that exists stays as it is.
    fn create_private_dir(&self, path: &Path) -> io::Result<()>;
    /// Create a new file with mode `0o600`, write the text, and sync it.
    fn create_private_file(&self, path: &Path, text: &str) -> io::Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn remove_file(&self, path: &Path) -> io::Result<()>;
    /// Remove each regular file in `dir` whose name starts with `TEMP_PREFIX`.
    /// A link, a folder, and each other file stay, and jag does not follow a
    /// link. A missing folder is fine. Returns the count of removed files.
    fn remove_stale_temporary_files(&self, dir: &Path) -> io::Result<usize>;
}

/// The production file system.
pub struct RealFs;

struct RealOpenFile(File);

impl OpenFile for RealOpenFile {
    fn mode(&self) -> io::Result<u32> {
        Ok(self.0.metadata()?.mode())
    }

    fn read_to_string(&mut self) -> io::Result<String> {
        let mut text = String::new();
        self.0.read_to_string(&mut text)?;
        Ok(text)
    }
}

impl Fs for RealFs {
    fn open(&self, path: &Path) -> io::Result<Box<dyn OpenFile>> {
        Ok(Box::new(RealOpenFile(File::open(path)?)))
    }

    fn is_symlink(&self, path: &Path) -> io::Result<bool> {
        match fs::symlink_metadata(path) {
            Ok(metadata) => Ok(metadata.file_type().is_symlink()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn is_special(&self, path: &Path) -> io::Result<bool> {
        match fs::metadata(path) {
            Ok(metadata) => Ok(!metadata.file_type().is_file()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn create_private_dir(&self, path: &Path) -> io::Result<()> {
        match DirBuilder::new().mode(0o700).create(path) {
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            other => other,
        }
    }

    fn create_private_file(&self, path: &Path, text: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)?;
        file.write_all(text.as_bytes())?;
        file.sync_all()
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        fs::rename(from, to)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn remove_stale_temporary_files(&self, dir: &Path) -> io::Result<usize> {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(0),
            Err(error) => return Err(error),
        };
        let mut removed = 0;
        for entry in entries {
            let entry = entry?;
            if !entry
                .file_name()
                .as_bytes()
                .starts_with(TEMP_PREFIX.as_bytes())
            {
                continue;
            }
            // `DirEntry::file_type` is the type of the entry itself: a link
            // is a link, and jag never follows it.
            if !entry.file_type()?.is_file() {
                continue;
            }
            match fs::remove_file(entry.path()) {
                Ok(()) => removed += 1,
                // Another process removed it, or renamed it over `config.yml`.
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::symlink;

    use super::*;

    #[test]
    fn it_removes_each_regular_temporary_file_and_nothing_else() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let target = outside.path().join("target.txt");
        fs::write(&target, "target").unwrap();
        fs::write(dir.path().join("config.yml.tmp-1-aa"), "old").unwrap();
        fs::write(dir.path().join("config.yml.tmp-"), "old").unwrap();
        fs::write(dir.path().join("config.yml"), "keep").unwrap();
        fs::write(dir.path().join("config.yml.tmp"), "keep").unwrap();
        fs::write(dir.path().join("other.txt"), "keep").unwrap();
        fs::create_dir(dir.path().join("config.yml.tmp-dir")).unwrap();
        symlink(&target, dir.path().join("config.yml.tmp-link")).unwrap();
        let removed = RealFs.remove_stale_temporary_files(dir.path()).unwrap();
        assert_eq!(removed, 2);
        let mut left: Vec<String> = fs::read_dir(dir.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        left.sort();
        assert_eq!(
            left,
            [
                "config.yml",
                "config.yml.tmp",
                "config.yml.tmp-dir",
                "config.yml.tmp-link",
                "other.txt"
            ]
        );
        assert_eq!(fs::read_to_string(&target).unwrap(), "target");
    }

    #[test]
    fn a_missing_folder_has_nothing_to_remove() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing");
        assert_eq!(RealFs.remove_stale_temporary_files(&missing).unwrap(), 0);
    }
}
