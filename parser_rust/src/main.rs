extern crate serde;
extern crate serde_json;

use serde::de::Deserialize;
use serde_json::{Value, Deserializer};

fn main() {
    let mut deserializer = Deserializer::from_reader(std::io::stdin());
    deserializer.disable_recursion_limit();
    let v = Value::deserialize(&mut deserializer).unwrap();
    println!("{}", v);
}
