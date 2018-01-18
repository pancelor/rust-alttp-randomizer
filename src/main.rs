extern crate env_logger;
extern crate group_by;
// #[macro_use]
// extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rand;

mod items;
mod locations;
mod locations2;
mod zones;
mod dungeons;

mod medallions;
mod world;
mod generator;
mod glue;
mod dive;
mod connections;
mod logic;

fn main() {
  env_logger::init().unwrap();

  use items::*;
  let advancement_items = vec![
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    Bow,
    Hammer,
    Lamp,
    BigKeyD1,
    MapD1,
    CompassD1,
  ];

  let junk_items = vec![
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
  ];

  // need 14 + prize + 5 overworld = 20 items total

  let mut rng = rand::thread_rng();

  let sim_count = 1;
  for _ in 0..sim_count {
    let world = generator::generate_world(&advancement_items, &junk_items, &mut rng);
    info!("{:?}", world);
    if !generator::can_win(&world) {
      panic!("uh oh, this world isn't beatable");
    }
  }
}
