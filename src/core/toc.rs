use schema::Schema;
use std::fs;
use consts;

pub struct TOC {
    schema: Schema,
    generation: i32,
    version: i32,
}

impl TOC {
    fn new(schema: Schema, generation: i32, version: i32) -> Self {
        TOC {
            schema: schema,
            generation: generation,
            version: version
        }
    }

    // index name creates with _filename
    fn create(&mut self, storage: Storage, index_name:String) -> Result<()> {
        let path = fs::read_dir(storage.get_path()).unwrap();
        for path in paths {
            let name = path.unwrap().path().display()
            if name.starts_with("_") {
                fs.delete_file(name)
            }
        }

        let mut new_index_name = index_name
        if new_index_name.is_empty() {
            new_index_name = DEF_INDEX_NAME.to_string()
        }
        self.create_toc_file(new_index_name)

    }

    fn create_toc_file(&self, index_name:String, generation: i32) -> bool {
        let toc_name = format!("_{0}_{1}", index_name, generation)
        let toc_temp_name = format!("_{0}_{1}", index_name, generation)
        fs.write(toc_temp_name)
        // dumping of schema items
        for schema in self.schema.items() {

        }

        // TODO: write generations and schema objects
    }
}