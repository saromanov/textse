use std::collections::HashMap;

use tantivy::schema::*;
use tantivy::Index;

struct Engine {
    create_index(&mut self name String, idx Index) -> Option<bool>
    delete_index(self name) -> Option<bool>
    get_index(self name) -> Option<Index>
}

impl HashEngine for Engine{
    map: HashMap<String, Index>
    pub fn create_index(&mut self name String, idx Index) -> Option<bool> {
        Some(true)
    }
}