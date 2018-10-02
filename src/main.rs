/*#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
*/
use std::fmt;
use std::fs::File;
use std::error::Error;

use textse::core::*;

mod schema;


type Result<T> = std::result::Result<T, DocumentError>;

#[derive(Debug, Clone)]
struct DocumentError{
    side:i32,
}

impl Error for DocumentError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}


fn get_version(num:i32) -> Result<String> {
    let mut contents = String::new();
    //Ok(contents);
    Err(DocumentError{side:2})
}
// https://github.com/serde-rs/serde


struct Card;

struct Bang<T>{
    value: T,
}

impl <T> Bang<T> {
    pub fn get_value(&self) -> &T {
        &self.value
    }
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Rectangle{
    length:f64,
    height:f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

fn get_aria<T:HasArea> (punk: T) {
    println!("{:?}", punk.area());
}

fn main() {
    let mut name = String::from("Peter");
    name.push('s');
    let age = 27;
    println!("{:?}", name);
    println!("Hello, world!");

    let f = File::open("Cargo.lock");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let good = match get_version(5) {
        Err(error) => {
             panic!("There was a problem opening the file: {:?}", error)
        }
        _ => println!("no error"),
    };

    let _char: Bang<char> = Bang{value:'a'};
    let olo: Bang<i64> = Bang{value:5};

    let schema = Schema::new()
    schema.add_text_field("title", TokenizerPunkt::new().add_sorted().filter())
    schema.create()
    schema.is_created()

    // creating of index
    let idx = Index::create_index("title", 1)
    // TODO
    let data = TextSe::new(schema:schema, index: idx)
    let document = Document::new()
    document.add_text(schema, "This is new document")
    document.add_text(schema, "This is second document")
    data.add_document(document)

}
