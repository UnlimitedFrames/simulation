mod structs;
use rand::{self, seq::SliceRandom, Rng};
use rnglib::{Language, RNG};
use structs::Person;
use uuid::Uuid;

fn main() {
    let name_rng = RNG::try_from(&Language::Goblin).unwrap();

    let mut rng = rand::thread_rng();
    let mut people: Vec<Person> = Vec::new();
    let names: Vec<String> = name_rng.generate_names(20000, true);
    for x in 0..100 {
        people.insert(
            x,
            Person {
                first_name: names.choose(&mut rng).unwrap().to_string(),
                last_name: names.choose(&mut rng).unwrap().to_string(),
                id: Uuid::new_v4(),
                age: rng.gen_range(0..40000),
            },
        );
    }
    println!("{:?}", people);
    run_day(&mut people);
    println!("{:?}", people);
}

fn run_day(people: &mut Vec<Person>) {
    for mut person in people {
        person.age = person.age + 1;
    }
}
