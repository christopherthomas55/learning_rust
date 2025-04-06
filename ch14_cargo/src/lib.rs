//! # ch14_cargo
//!
//! `ch14_cargo` is a set of example files to learn how the rust cargo tool works

// Above we have comments that are on the owning item (usually module or root lib file) a
//

// This allows us to avoid using nested ch14_cargo::kinds::PrimaryColor like calls anywhere
// In general we avoid the module structure to make these easier for others to use
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


// Example of documentation comments
/// Adds one to number given
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ch14_cargo::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// You can then run `cargo doc` to generate docs... --open option will open in browser

// Another cool thing - the examples in ``` are run in tests too. Wow!


pub mod kinds {
    /// Primary colors in RYB color model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary colors in RYB color model
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    }
}
