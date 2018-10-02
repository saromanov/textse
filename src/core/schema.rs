pub struct Schema;

// Schema should be multithreading
// for write it to index


impl Schema {
    fields: Vec<Field>;
    fileds_map: HashMap<String, Field>;

     // new provides crete of the new schema
    // This is main method for create a new schema for indexing
    fn new() -> Schema {
        Schema{
            fields:: Vec::new(),
            fileds_map: HashMap::new(),
        }
    }
    fn add_text_field(&mut self, name:String, typeText:String) {
        let field_name = String::from(field_name_str);
        let field = Field(field_name, typeText)
        self.add_field(field)
        // insert on the case if field doesn't exist
        self.fileds_map.entry(field_name).or_insert(field_name, field)
    }

    fn add_i64_field(&mut self, name:String, typeText: String) {
        let field_name = String::from(field_name_str);
        let field = Field(field_name, typeText)
        self.add_field(field)
    }

    fn remove_field(&mut self, name:String) {
        self.remove_field(name)
    }

    // get_names returns defined names fo fields
    fn get_names(&self) -> Vec<String> {
        self.fileds_map.keys().to_vector()
    }

    fn get_field(&self, name:String) -> Field {
        self.fileds_map.get(name)
    }

    // is_created checks if schema is created
    fn is_created() -> bool {
        true;
    }

    fn add_field(&self, f: Field) {
        self.fields.push(f)
    }
}