use std::collections::HashMap;

// Index provides ccore for indexing 
// heart of this its engine 
// schema is defined schema
pub struct Index {
    index_collection: Engine
    schema: Schema
    pub fn create_index(&mut self name String, idx i64) -> Option<i32>;
    pub fn delete_index(self name) -> Option<i32>;
    pub fn add_document(&self, document:Document) -> Result<bool, DocumentError>;
}

enum DocumentError {
        ValidateError(widget_name: String) {
            description("The widget could not be found")
            display(r#"The widget "{}" could not be found"#, widget_name)
        }
    }

impl Index {
    pub fn create_index(&mut self name String, idx i64){
        self.index_collection.push(name, idx)
    }

    pub fn delete_index(self name) -> Option<i32> {
        self.index_collection.delete(name)
    }

     pub fn add_document(&self, document:Document) -> Result<bool, DocumentError> {
        if !document.validate_document()
    }
}