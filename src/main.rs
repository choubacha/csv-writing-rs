extern crate csv;
#[macro_use]
extern crate fake;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use csv::WriterBuilder;
use rand::{thread_rng, Rng};
use std::io;

#[derive(Serialize, Deserialize)]
struct Outer {
    id: u64,
    name: String,
    data: Inner,
}

#[derive(Serialize, Deserialize)]
struct Inner {
    value: String,
    num: u64,
}

fn generate() -> Vec<Outer> {
    let mut rng = thread_rng();
    (0..40)
        .map(|id| Outer {
            id,
            name: fake!(Name.name),
            data: Inner {
                value: fake!(Lorem.sentence(4, 6)),
                num: rng.gen_range(0, 1_000_000),
            },
        })
        .collect()
}

fn main() {
    let rows = generate();
    let stdout = io::stdout();
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(stdout);
    for row in rows {
        writer.serialize(row).unwrap();
    }
}
