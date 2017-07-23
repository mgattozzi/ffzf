use std::path::PathBuf;
use std::fs::DirEntry;
use std::convert::TryFrom;

pub struct Entry {
    symlink: bool,
    dir: bool,
    path: PathBuf,
}

impl Entry {
    pub fn new(symlink: bool, dir: bool, path: PathBuf) -> Self {
        Entry { symlink, dir, path }
    }

    pub fn path(self) -> PathBuf {
        self.path
    }

    pub fn is_dir(&self) -> bool {
        self.dir
    }

    pub fn is_symlink(&self) -> bool {
        self.symlink
    }
}

impl TryFrom<DirEntry> for Entry {
    type Error = ::std::io::Error;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        let path = entry.path();
        let ftype = entry.file_type()?;
        let dir = ftype.is_dir();
        let symlink = ftype.is_symlink();

        Ok(Entry::new(symlink, dir, path))
    }
}
