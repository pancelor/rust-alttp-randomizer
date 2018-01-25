#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use super::zones;
use super::zones::*;
use super::items::Item;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum Location2 {
  TempOverworld1,
  TempOverworld2,
  TempOverworld3,
  TempOverworld4,
  TempOverworld5,
  TempOverworld6,
  TempOverworld7,
  TempOverworld8,
  TempOverworld9,
  TempOverworld10,

  EasternPalaceCompassChest,
  EasternPalaceBigChest,
  EasternPalaceCannonballChest,
  EasternPalaceBigKeyChest,
  EasternPalaceMapChest,
  EasternPalaceKeyPot,
  EasternPalaceKeyEyegore,
  EasternPalaceArmosKnights,
  EasternPalacePrize,

  TowerOfHeraMapChest,
  TowerOfHeraBasementCage,
  TowerOfHeraBigKeyChest,
  TowerOfHeraCompassChest,
  TowerOfHeraBigChest,
  TowerOfHeraMoldorm,
  TowerOfHeraPrize,

  PalaceOfDarknessBigKeyChest,
  PalaceOfDarknessTheArenaLedge,
  PalaceOfDarknessTheArenaBridge,
  PalaceOfDarknessBigChest,
  PalaceOfDarknessCompassChest,
  PalaceOfDarknessHarmlessHellway,
  PalaceOfDarknessStalfosBasement,
  PalaceOfDarknessDarkBasementLeft,
  PalaceOfDarknessDarkBasementRight,
  PalaceOfDarknessMapChest,
  PalaceOfDarknessDarkMazeTop,
  PalaceOfDarknessDarkMazeBottom,
  PalaceOfDarknessShooterRoom,
  PalaceOfDarknessHelmasaurKing,
  PalaceOfDarknessPrize,

  // DesertPalacePrize,
  // IcePalacePrize,
  // MiseryMirePrize,
  // SkullWoodsPrize,
  // SwampPalacePrize,
  // ThievesTownPrize,
  // TurtleRockPrize,
}
pub use self::Location2::*;

pub fn get_all_locations() -> Vec<Location2> {
  vec![
    TempOverworld1,
    TempOverworld2,
    TempOverworld3,
    TempOverworld4,
    TempOverworld5,
    TempOverworld6,
    TempOverworld7,
    TempOverworld8,
    TempOverworld9,
    TempOverworld10,

    EasternPalaceCompassChest,
    EasternPalaceBigChest,
    EasternPalaceCannonballChest,
    EasternPalaceBigKeyChest,
    EasternPalaceMapChest,
    EasternPalaceKeyPot,
    EasternPalaceKeyEyegore,
    EasternPalaceArmosKnights,
    EasternPalacePrize,

    TowerOfHeraMapChest,
    TowerOfHeraBasementCage,
    TowerOfHeraBigKeyChest,
    TowerOfHeraCompassChest,
    TowerOfHeraBigChest,
    TowerOfHeraMoldorm,
    TowerOfHeraPrize,

    PalaceOfDarknessBigKeyChest,
    PalaceOfDarknessTheArenaLedge,
    PalaceOfDarknessTheArenaBridge,
    PalaceOfDarknessBigChest,
    PalaceOfDarknessCompassChest,
    PalaceOfDarknessHarmlessHellway,
    PalaceOfDarknessStalfosBasement,
    PalaceOfDarknessDarkBasementLeft,
    PalaceOfDarknessDarkBasementRight,
    PalaceOfDarknessMapChest,
    PalaceOfDarknessDarkMazeTop,
    PalaceOfDarknessDarkMazeBottom,
    PalaceOfDarknessShooterRoom,
    PalaceOfDarknessHelmasaurKing,
    PalaceOfDarknessPrize,


    // DesertPalacePrize,
    // IcePalacePrize,
    // MiseryMirePrize,
    // SkullWoodsPrize,
    // SwampPalacePrize,
    // ThievesTownPrize,
    // TurtleRockPrize,
  ]
}
