//
// field.rs represents implementation
// of the field for schema and serialization
// and deserialization on the fields based on type

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum FieldType {
   I64,
   STRING,
   F64
}

struct Field {
    name: String
    field_type: FieldType
    timestamp: i64
}

impl Field {
    pub fn new(name String, field_type FieldType) -> Field {
        let start = SystemTime::now();
        let unix = start.duration_since(UNIX_EPOCH).as_secs();
        Field {
            name: name,
            field_type: FieldType,
            timestamp: unix
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }

     pub fn field_type(&self) -> &i64 {
        &self.timestamp
    }

}