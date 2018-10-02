pub struct Term<T> {
    field: String,
    text: String.
}

impl Term<T> {
    fn new(field: String, text:String) -> Self {
        Self{
            field: field,
            text:text,
        }
    }

}