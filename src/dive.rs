#![allow(unused_imports)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::{HashMap, BTreeSet};
use rand::{Rng, ThreadRng};
use super::{medallions, logic, locations2, items, zones, dungeons};
use super::zones::Zone;
use super::connections::*;
use super::items::Item;
use super::dungeons::Dungeon;
use super::world::Assignments;
use super::locations2::Location2;
use group_by;

// All `Dive`s are greedy wrt items
// `Dive.explore` expands over itemdoor boundaries, but not keylock boundaries (besides the one door passed as an argument)

#[derive(Eq, Clone, Debug)]
pub struct Dive {
  pub zones: BTreeSet<Zone>,
  pub items: Vec<Item>, // includes big/small keys
  pub open_doors: BTreeSet<KeyDoor>, // all open keydoors on the entire map, not just the immediately accessible ones
}

impl PartialEq for Dive {
  fn eq(&self, other: &Self) -> bool {
    self.zones == other.zones
    && self.open_doors == other.open_doors
    && self.items.clone().sort() == other.items.clone().sort()
  }
}

impl Hash for Dive {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.zones.hash(state);
    self.open_doors.hash(state);
    self.items.clone().sort().hash(state)
  }
}

impl Dive {
  pub fn new(
    items: Vec<Item>,
    assignments: &Assignments,
  ) -> Self {
    let mut me = Self {
      zones: BTreeSet::new(),
      items,
      open_doors: BTreeSet::new(),
    };
    me.loot_zone(Zone::TempEastLightWorld, &assignments);
    me.explore(&assignments);
    me
  }

  pub fn hash_value(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.hash(&mut hasher);
    hasher.finish()
  }

  fn dungeons_i_own_keys_for(&self) -> BTreeSet<&Dungeon> {
    dungeons::ALL.iter()
      .filter(|&&dungeon| {
        let target_key = WG.key_from_dungeon(dungeon);
        let num_keys = self.items.iter()
          .filter(|&&item| item == target_key)
          .count();
        let num_opened_doors = self.open_doors.iter()
          .filter(|&&kdoor| WG.dungeon_from_keydoor(kdoor) == dungeon)
          .count();
        num_opened_doors < num_keys
      }).collect()
  }

  pub fn key_frontier(&self) -> BTreeSet<KeyDoor> {
    let dungeons_i_own_keys_for = self.dungeons_i_own_keys_for();
    self.zones.iter()
      .flat_map(|&zone| WG.keyfrontier_from_zone(zone))
      .filter(|&&kdoor| dungeons_i_own_keys_for.contains(&WG.dungeon_from_keydoor(kdoor)))
      .filter(|&&kdoor| !self.open_doors.contains(&kdoor))
      .cloned()
      .collect()
  }

  /// all reachable itemdoors, filtered to ones that we haven't opened yet.
  ///   This DOES include itemdoors we currently can't pass through
  ///   Only used internally when `explore()`ing
  /// Note: there's still some internal confusion on reversible doors;
  ///   this function currently will include 1-way idoors that look _into_
  ///   self but are not actually reachable from self
  fn item_frontier(&self) -> Vec<ItemDoor> {
    self.zones.iter()
      .flat_map(|&zone| WG.itemfrontier_from_zone(zone))
      .filter(|&&idoor| !(self.zones.contains(&idoor.zone1) && self.zones.contains(&idoor.zone2)))
      .cloned()
      .collect()
  }

  pub fn explore_keydoor(&mut self, door: KeyDoor, assignments: &Assignments) {
    trace!("fn explore_keydoor()");
    self.open_keydoor(door, &assignments);
    self.explore(&assignments);
  }

  pub fn explore(&mut self, assignments: &Assignments) {
    // assumes self is already greedy (i.e. wont re-explore self.zones)
    debug!("Explore:\n\titems={:?}", self.items);
    trace!("fn explore(\n\tself={:?},\n\tassignments={:?}\n)", self, assignments);

    let mut num_passes = 0;
    loop {
      num_passes += 1;
      // gotta set item_frontier here b/c this algo will otherwise fail if there are two ItemDoors in sequence (e.g. with a key spoke also connected to the hub of those three connections)
      // TODO: make it smarterrr probs (i.e. that reverted commit)
      let item_frontier: Vec<ItemDoor> = self.item_frontier();
      if !self.do_one_exploration_pass_on_frontier(&item_frontier, &assignments) {
        break;
      }
    }
    debug!("Explore finished after {} passes", num_passes);
  }

  /// Returns whether any new zones were added during this pass
  /// TODO: it's probably a lot better to instead return a list of new idoors; esp when the dive is beatable
  fn do_one_exploration_pass_on_frontier(&mut self, ifront: &Vec<ItemDoor>, assignments: &Assignments) -> bool {
    trace!("do_one_exploration_pass_on_frontier(\n\tifront={:?},\n)", ifront);

    let mut new_zones = false;
    for &current_edge in ifront.iter() {
      if !current_edge.can_pass(&self.items) { continue; }
      let zone: Zone = if !self.zones.contains(&current_edge.zone2) {
        current_edge.zone2
      } else if current_edge.reversible && !self.zones.contains(&current_edge.zone1) {
        current_edge.zone1
      } else {
        continue;
      };
      new_zones = true;
      self.loot_zone(zone, &assignments);
    }
    new_zones
  }

  fn open_keydoor(&mut self, door: KeyDoor, assignments: &Assignments) {
    trace!("fn open_keydoor(\n\tself={:?},\n\tdoor={:?}\n\tassignments={:?}\n)", self, door, assignments);

    // sanity check door is in frontier
    let key_frontier: BTreeSet<KeyDoor> = self.zones.iter()
      .flat_map(|&zone| WG.keyfrontier_from_zone(zone))
      .cloned()
      .collect();
    if !key_frontier.contains(&door) { panic!("trying to cross through a door not in the frontier"); }

    // sanity check we have a key to open the door
    let dungeon = WG.dungeon_from_keydoor(door);
    if !self.dungeons_i_own_keys_for().contains(&dungeon) {
      panic!("Trying to open a door you don't have a key for");
    }

    let is_new: bool = self.open_doors.insert(door);
    if !is_new {
      panic!("trying to re-open an open door");
    }

    // We only want to `insert` the items from unexplored zones (b/c `self.items` is a HashMultiSet)
    let new_zone: Zone;
    if !self.zones.contains(&door.zone1) {
      new_zone = door.zone1;
    } else if !self.zones.contains(&door.zone2) {
      new_zone = door.zone2;
    } else {
      trace!("opened a useless door (e.g. the left<->right keydoor in GT");
      return;
    }
    self.loot_zone(new_zone, &assignments);
  }

  fn loot_zone(&mut self, zone: Zone, assignments: &Assignments) {
    // TODO: make self.items a method, and calc it on the fly from zones? makes for easier debugging... yeah lets do it
    if !self.zones.insert(zone) {
      panic!("Trying to re-loot a zone");
    }

    WG.locations_from_zone(zone).iter()
      .filter_map(|loc| assignments.get(&loc))
      .for_each(|&item| self.items.push(item));

    debug!("Looting {:?}", zone);
    // debug!("Looting:\n\tzone={:?}\n\t(post) self.items={:?}", zone, self.items);
    trace!("fn (post) loot_zone(\n\tself={:?},\n\tzone={:?}\n\tassignments={:?}\n)", self, zone, assignments);
  }
}
