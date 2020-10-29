// automatically generated by the FlatBuffers compiler, do not modify
#[derive(PartialEq,Clone)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[derive(PartialEq,Clone,Default)]
pub struct NamedAnimal {
    pub name: String,
    pub age: Option<i16>,
}

#[derive(PartialEq,Clone,Default)]
pub struct Person {
    pub name: String,
    pub address: Option<String>,
    pub age: Option<i16>,
    pub length: Option<u64>,
    pub favorite_color: Color,
}

#[derive(PartialEq,Clone,Default)]
pub struct Product {
    pub label: Option<String>,
    pub price: Option<i32>,
}

#[derive(PartialEq,Clone)]
enum Item {
    Product,
    Person,
}
