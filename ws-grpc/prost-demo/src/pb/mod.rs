pub mod animals;
mod oneof;

#[cfg(test)]
mod oneof_tests {
    use crate::pb::oneof::animal::AnimalType;
    use crate::pb::oneof::{Animal, Cat};
    use prost::Message;

    #[test]
    fn test_oneof() {
        let mut animal = Animal::default();
        animal.name = "Tom".to_string();
        animal.age = 3;
        animal.animal_type = Some(AnimalType::Cat(Cat { has_tail: true }));

        let mut buf = Vec::new();
        animal.encode(&mut buf).unwrap();

        let decoded_animal = Animal::decode(&buf[..]).unwrap();
        assert_eq!(animal, decoded_animal);
    }
}
