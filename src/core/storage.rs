use std::fs

struct StorageInner;

impl StorageInner {
    path: String,
    read_only: bool,
}

struct Storage {
    storage: StorageInner;
    read_only: bool
}

impl Storage {
    pub fn new(&self, path:String, read_only: bool) -> Storage {
        let storage = new StorageInner {
            path: path,
            read_only: read_only,
        }

        Storage{storage: storage}
    }

    fn get_path(&self) -> String {
        self.storage.path
    }
}