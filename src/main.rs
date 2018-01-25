extern crate env_logger;
extern crate group_by;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rand;

use std::env;

mod items;
mod locations;
mod locations2;
mod zones;
mod dungeons;

mod medallions;
mod world;
mod generator;
mod dive;
mod connections;
mod logic;


fn main() {
  env_logger::init().unwrap();
  real_main();
}

#[allow(dead_code)]
#[allow(unused_imports)]
fn temp_main() {
  use locations2::*;
  use items::*;
  use zones::*;
  use dungeons::*;
  use connections::{WG, KeyDoor};
  use dive::Dive;

  let locs1 = WG.locations_from_dungeon(EasternPalace);
  let locs2 = WG.locations_from_dungeon(PalaceOfDarkness);
  println!("EP={:?} PD={:?}", locs1, locs2);
}

#[allow(dead_code)]
#[allow(unused_imports)]
fn real_main() {
  use connections::WG;
  use items::*;

  let mut advancement_items = vec![
    FireRod,
    Bow,
    Hammer,
    Lamp,
  ];

  let mut dungeon_items = vec![
    MapP3,
    MapP1,
    MapD1,

    CompassP3,
    CompassP1,
    CompassD1,

    KeyP3,
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,

    BigKeyP3,
    BigKeyP1,
    BigKeyD1, // items at this end will be placed first
  ];

  let keysanity = env::var("KEYSANITY").is_ok();
  if keysanity {
    advancement_items.append(&mut dungeon_items);
  }
  debug!("keysanity={}, dungeon_items={:?}", keysanity, dungeon_items);

  let mut junk_items = vec![];
  junk_items.extend((0..).take(16).map(|_| Heart));

  let mut rng = rand::thread_rng();

  let sim_count = match env::var("NSIM") {
    Ok(val) => val.parse().expect("bad NSIM format"),
    Err(_) => 1,
  };
  for ii in 0..sim_count {
    info!("sim #{:?}", ii);
    let world = generator::generate_world(advancement_items.clone(), dungeon_items.clone(), junk_items.clone(), &mut rng);

    info!("worldgen finished: {:?}", world);
    if !generator::can_win(&world) {
      println!("{:?}", world);
      panic!("uh oh, this world isn't beatable");
    }
  }
}

// TODO rm
#[allow(dead_code)]
fn key_in_dark_maze(world: &world::World) -> bool{
  use locations2::*;
  use items::*;
  (
    world.get(&PalaceOfDarknessBigChest) == Some(&KeyD1)
    || world.get(&PalaceOfDarknessDarkMazeTop) == Some(&KeyD1)
    || world.get(&PalaceOfDarknessDarkMazeBottom) == Some(&KeyD1)
  )
}
