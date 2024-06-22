use crate::pb::animals::{Animal, AnimalType};
use prost::Message;

mod pb;

fn main() {
    let mut animal = Animal::default();
    animal.name = "Tom".to_string();
    animal.age = 3;
    animal.animal_type = AnimalType::Cat as i32;

    let mut buf = Vec::new();
    animal.encode(&mut buf).unwrap();

    let decoded_animal = Animal::decode(&buf[..]).unwrap();
    assert_eq!(animal, decoded_animal);
    println!("{:?}", animal);
}
