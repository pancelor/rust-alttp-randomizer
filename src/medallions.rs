#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum MedallionLock {
  BombosLock,
  EtherLock,
  QuakeLock,
}

pub fn get_all_medallions() -> Vec<MedallionLock> {
  use self::MedallionLock::*;
  vec![
    BombosLock,
    EtherLock,
    QuakeLock,
  ]
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct EntranceConfig {
  pub turtle_rock: MedallionLock,
  pub misery_mire: MedallionLock,
}
