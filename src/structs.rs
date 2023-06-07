use uuid::Uuid;

#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub id: Uuid,
    pub age: u16,
}
