use schema::Schema;

type Fields = Vec<Field>;

struct Document;

impl Document {
    entries: Fields

    pub fn add_text(&self, schema:Schema, field:String) {
        let field = Field::new(name, field)
        self.entries.push(field)
    }
    pub fn len(&self) -> usize {
        self.entries.length()
    }
    pub fn is_empty(&self) -> Option<bool> {
        if self.entries.is_empty() == 0 {
            Some("list is empty")
        }

        None
    }
    pub fn dedup(&mut self) -> bool {
        self.entries.dedup()
    }
    pub fn filter<P: Fn(Field) -> bool>(&mut self, predicate: P) {
        self.entries.retain(|field_value| predicate(self.entries.field()));
    }
    pub fn get_all(&self, field: Field) -> Vec<&Value> {
        self.entries
            .iter()
            .filter(|field_value| field_value.field() == field)
            .map(|field_value| field_value.value())
            .collect()
    }

    // validate_document provides validation
    // of current document object
    pub fn validate_document(&self) -> bool {
        if self.entries.len() == 0 {
            false
        }
        true
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Document) -> bool {
        let mut self_field_values = self.field_values.clone();
        let mut other_field_values = other.field_values.clone();
        self_field_values.sort();
        other_field_values.sort();
        self_field_values.eq(&other_field_values)
    }
}