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
    for x in 0..1000 {
        people.insert(
            x,
            Person {
                first_name: names.choose(&mut rng).unwrap().to_string(),
                last_name: names.choose(&mut rng).unwrap().to_string(),
                id: Uuid::new_v4(),
                age: rng.gen_range(0..40000),
                attributes: Vec::new(),
                alive: true,
            },
        );
    }
    // for person in &people {
    //     println!("{} : {}", person.first_name, person.age / 365);
    // }
    for _x in 0..1000000 {
        run_day(&mut people);
    }
    let mut total_age = 0;
    for person in &people {
        total_age += person.age
    }

    print!("{}", total_age / people.len() as u64);
}

fn run_day(people: &mut Vec<Person>) {
    let mut rng = rand::thread_rng();

    for mut person in people {
        if person.alive {
            person.age = person.age + 1;

            for attr in &mut person.attributes {
                attr.run_day();
            }

            let mut death_chance: f64 = 0.0000014;
            if person.age > 18262 {
                death_chance = death_chance + (0.000002 * (person.age - 18262) as f64 / 365 as f64);
            }
            let roll: f64 = rng.gen();
            if roll < death_chance {
                // println!("{} : {}", person.first_name, person.age / 365);
                person.alive = false;
            }
        }
    }
}
