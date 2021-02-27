extern crate rand;

use rand::Rng;

use std::fs::File;
use std::io::{BufRead, BufReader};

struct World {
    male_first_names: Vec<String>,
    female_first_names: Vec<String>,
    last_names: Vec<String>,
    locations: Vec<String>,
    patronships: Vec<String>,
    date_range: (usize, usize)
}

impl World {
    fn new(db_dir: String) -> World {
        World {
            male_first_names: read_db(format!("{}/male_first_names", db_dir)),
            female_first_names: read_db(format!("{}/female_first_names", db_dir)),
            last_names: read_db(format!("{}/last_names", db_dir)),
            locations: read_db(format!("{}/locations", db_dir)),
            patronships: read_db(format!("{}/patronships", db_dir)),
            date_range: (600, 1300)
        }
    }
}

struct Saint {
    first_name: String,
    last_name: String,
    location: String, // Used for things like "Saint Maria of Seville"
    patron: Vec<String>, // Things they are a patron of.
    dob: usize, // Date of birth
    dod: usize, // Date of death
    story: Option<String>
}

impl Saint {
    fn new(world: World) -> Saint {
        let mut rng = rand::thread_rng();

        // Variables that will be used in both calculations and struct
        let first_name: String;
        let mut patron = Vec::new();
        let dob = rng.gen_range(world.date_range.0, world.date_range.1);

        // Calculate first name
        if rng.gen_range(0, 2) == 1 {
            first_name = rand_in_vec(world.male_first_names);
        } else {
            first_name = rand_in_vec(world.female_first_names);
        }

        // Calculate what they are patron of. 
        let mut patronships = world.patronships.clone();
        let amount_patron = rng.gen_range(1, 4);

        for _ in 0..amount_patron {
            let p = patronships.clone();
            let patron_item = rand_in_vec(p);
            patron.push(patron_item.clone());

            for i in 0..patronships.len() - 1 {
                if patronships[i] == patron_item {
                    patronships.remove(i);
                }
            }
        }

        let mut saint = Saint {
            first_name: first_name,
            last_name: rand_in_vec(world.last_names),
            location: rand_in_vec(world.locations),
            patron: patron,
            dob: dob,
            dod: dob + rng.gen_range(16, 80),
            story: None
        };

        saint.gen_story();
        saint
    }

    fn gen_story(&mut self) {
        let mut story = format!(
            "Saint {} {} of {} was born in {} living for {} years before dying of {} in {}.\n",
            self.first_name, self.last_name, self.location, self.dob, self.dod - self.dob, "unknown causes", self.dod
        );

        story = story + &format!("Saint {} {} of {} is the patron saint of ", self.first_name, self.last_name, self.location);

        if self.patron.len() == 1 {
            story = story + &format!("{}.", self.patron[0]);
        } else {
            for i in 0..self.patron.len() - 1 {
                if i == 0 {
                    story = story + &format!("{}", self.patron[i]);
                } else {
                    story = story + &format!(", {}", self.patron[i]);
                }
            }
            story = story + &format!(" and {}.", self.patron[self.patron.len() - 1]);
        }

        self.story = Some(story);
    }

    fn print(self) {
        if self.story == None {
            println!("{} {} of {}.", self.first_name, self.last_name, self.location);
            println!("Patron of {:?}", self.patron);
            println!("Born {} AD and died {} AD", self.dob, self.dod);
        } else {
            println!("{}", self.story.unwrap());
        }
    }
}

// Get a random item from a vector
fn rand_in_vec<T: Clone>(vec: Vec<T>) -> T {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0, vec.len());
    vec[i].clone()
}

fn read_db(path: String) -> Vec<String> {
    let mut data = Vec::new();
    let file = File::open(path).unwrap();

    for line in BufReader::new(file).lines() {
        data.push(line.unwrap());
    }

    data
}

fn main() {
    let world = World::new("./db".to_string());
    let saint = Saint::new(world);
    saint.print();
}   