//! # Art
//! 
//! A library for modeling artictic concepts

// to give the user access to the enums without haveing to reference
// the respective modules
pub use self::kinds::PrimaryColors;
pub use self::kinds::SecondaryColors;
pub use self::utils::mix;

pub mod kinds{
  /// The primary colors according to RYB.
  pub enum PrimaryColors{
    Red,
    Yellow,
    Blue,
  }

  /// The secondary colors according to RYB.
  pub enum SecondaryColors{
    Orange,
    Green,
    Purple,
  }
}

pub mod utils{
  use crate::kinds::*;

  /// Combines two primary colors in equal amounts to create a 
  /// secondary color.
  pub fn mix(c1: PrimaryColors, c2: PrimaryColors) -> SecondaryColors{
    // --snip--
    // ANCHOR_END: here
    SecondaryColors::Orange
    // ANCHOR: here
  }
}