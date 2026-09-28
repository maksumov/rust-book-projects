// The four lint levels, weakest to strongest: allow (silence),
// warn (warning; the default for most lints), deny (compile error),
// forbid (deny + a later allow is itself an error). Any lint name
// works with any level, e.g. #![warn(missing_docs_in_private_items)].
// Details: GLOSSARY.md, "Lint levels".
#![warn(missing_docs)]

//! # Art
//!
//! A library for modeling artistic concepts.
//!
//! # Examples
//!
//! ```
//! use art::{mix, PrimaryColor};
//!
//! let _ = mix(PrimaryColor::Red, PrimaryColor::Yellow);
//! ```

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

/// Color kind definitions for the RYB color model.
pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum PrimaryColor {
        /// Red; mixing with Yellow yields Orange, with Blue yields Purple.
        Red,

        /// Yellow; mixing with Red yields Orange, with Blue yields Green.
        Yellow,

        /// Blue; mixing with Yellow yields Green, with Red yields Purple.
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        /// The result of mixing Red and Yellow.
        Orange,

        /// The result of mixing Yellow and Blue.
        Green,

        /// The result of mixing Red and Blue.
        Purple,
    }
}

/// Utility functions for combining colors.
pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    ///
    /// # Examples
    ///
    /// ```
    /// use art::{mix, PrimaryColor, SecondaryColor};
    ///
    /// let orange = mix(PrimaryColor::Red, PrimaryColor::Yellow).unwrap();
    /// assert_eq!(orange, SecondaryColor::Orange);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err` if both primary colors are identical, since mixing
    /// a color with itself cannot produce a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Result<SecondaryColor, String> {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow)
            | (PrimaryColor::Yellow, PrimaryColor::Red) => Ok(SecondaryColor::Orange),

            (PrimaryColor::Yellow, PrimaryColor::Blue)
            | (PrimaryColor::Blue, PrimaryColor::Yellow) => Ok(SecondaryColor::Green),

            (PrimaryColor::Red, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Red) => {
                Ok(SecondaryColor::Purple)
            }

            _ => Err(String::from(
                "Cannot mix identical primary colors into a secondary color",
            )),
        }
    }
}
