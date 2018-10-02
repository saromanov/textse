use std::fs
use std::path::Path;


type IOResult = std::io::Result<()>;

#[derive(Copy)]
struct StorageInner {
    path: String,
    read_only: bool
}

impl Clone for StorageInner {
    fn clone(&self) -> StorageInner { *self }
}

impl StorageInner {
    pub fn new(path:String, read_only: bool) -> StorageInner {
        StorageInner{path: path, read_only: bool}
    }

    fn create_dir(&self) -> IOResult {
        fs::create_dir_all(self.path)?;
        Ok(())
    }


    // open_dir provides opening of the dir with indexes
    fn open_dir(&self) -> IOResult {
        if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
    }

    fn remove_dir(&self) -> IOResult {
        if self.read_only {
            return
        }
         fs::remove_dir_all(self.path)?;
         Ok(())
    }

    // exist_dir return true if index directory is exist
    fn exist_dir(&self) -> bool {
        Path::new(self.path).exists()
    }
}
