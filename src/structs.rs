use uuid::Uuid;

#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub id: Uuid,
    pub age: u64,
    pub attributes: Vec<Attibute>,
    pub alive: bool,
}

#[derive(Debug)]
pub struct Attibute {
    pub name: String,
}

impl Attibute {
    pub fn run_day(&mut self) {}
}
