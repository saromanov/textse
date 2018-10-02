use std::fs;

// https://bitbucket.org/mchaput/whoosh/src/a16ebacb47191afaf2d1fda8b1fcd84e6981582f/src/whoosh/index.py?at=default&fileviewer=file-view-default

use index::IndexSchema


struct FileReader<T>;

impl <T> FileReader <T> {
    pub fn write() -> Result<path, ErrorFileWrite> {
        fs.write(self.path)
    }

    // creatign temp file before commit
    fn create_temp_file(path:String){
        fs.create_file(path)

    }
}