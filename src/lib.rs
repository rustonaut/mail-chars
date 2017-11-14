//!
//! provides char classification for mail related grammar parts / charset
//!
//! For example if a given char belongs characters valid in atext, ctext, dtext, token etc.
//!
//! The `Charsets` enum is used to determine which set of character is used.
//! To check if a `char` is in that set either use `Charset::contains(&self, char)`.
//! Or use `ch.is(charset)` which is provided through the `CharMatchExt` extension trait.
//!
//! # Alternative interface
//!
//! All enum variant are reexported under a module with the name of the rfc where
//! they are specified. E.g. `Charset::CText` is also available as `rfc5322::CText`.
//!
//!
//! # Example
//!
//! ```rust
//! use mail_chars::{rfc5322, rfc2045, CharMatchExt};
//!
//! fn is_token(input: &str) -> bool {
//!     input.chars().all(|ch| ch.is(rfc2045::Token))
//! }
//!
//! assert!(is_token("abc-def"));
//! assert!(!is_token("abc de"));
//!
//! fn simple_quote(input: &str) -> String {
//!     let mut out = String::with_capacity(input.len()+2);
//!     out.push('"');
//!     for ch in input.chars() {
//!         if ch.is_inkl_non_ascii(rfc5322::QTextWs) {
//!             out.push(ch);
//!         } else if ch == '\\' || ch == '"' {
//!             out.push('\\');
//!             out.push(ch);
//!         } else {
//!             panic!("wupsi dupsi that wont work ;=)");
//!         }
//!     }
//!     out.push('"');
//!     out
//! }
//!
//! assert_eq!(simple_quote("this \" is it"), "\"this \\\" is it\"");
//! ```

mod lookup;

/// A enum for the charsets represented through an internal lookup table
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum Charsets {
    /// qtext + ws basically anything which can appear in a quoted string which is not a quoted-pair
    ///
    /// **rfc: 5322**
    QTextWs = lookup::QC,

    /// ctext
    ///
    /// **rfc: 5322**
    CText = lookup::CT,

    /// dtext
    ///
    /// **rfc: 5322**
    DText = lookup::DT,

    /// atext
    ///
    /// **rfc: 5322**
    AText = lookup::AT,

    /// restricted-name-char subset of rfc2045 token with which ietf-tokens and iana-tokens have to comply
    ///
    /// **rfc: 6838** (related rfc2045)
    RestrictedToken = lookup::RT,

    /// token
    ///
    /// **rfc: 2045**
    ///
    /// Note there are multiple mail related definitions of token, this one is the rfc2045 based
    /// one.
    Token = lookup::TO
}

impl Charsets {

    /// returns true if the char is part of this set of chars
    #[inline]
    pub fn contains(&self, ch: char) -> bool {
        self.contains_lookup(ch, false)
    }

    /// returns true if the char is part of the set of chars or not an ascii char
    ///
    /// this is mainly meant to be used in combination with rfc6532 which extends
    /// all *text grammar parts/character sets to contain any non-us-ascii character
    #[inline]
    pub fn contains_or_non_ascii(&self, ch: char) -> bool {
        self.contains_lookup(ch, true)
    }

    fn contains_lookup(&self, ch: char, out_of_table_value: bool) -> bool {
        let index = ch as u32;
        if index < 0x80 {
            lookup::US_ASCII_LOOKUP[index as usize] & (*self as u8) != 0
        } else {
            out_of_table_value
        }
    }

    /// uses the internal lookup table to classify a char
    pub fn lookup(ch: char) -> LookupResult {
        let index = ch as u32;
        if index < 0x80 {
            LookupResult(Some(lookup::US_ASCII_LOOKUP[index as usize]))
        } else {
            LookupResult(None)
        }
    }
}

mod sealed{ pub trait Seal {} }
pub use self::sealed::Seal;
pub trait CharMatchExt: Seal+Copy {
    /// returns true if the char is a char belonging to the given charset
    fn is(self, charset: Charsets) -> bool;
    /// returns true if the char is a char belonging to the given charset or a non-us-ascii char
    fn is_inkl_non_ascii(self, charset: Charsets) -> bool;
}

impl Seal for char {}
impl CharMatchExt for char {
    #[inline]
    fn is(self, charset: Charsets) -> bool {
        charset.contains(self)
    }
    #[inline]
    fn is_inkl_non_ascii(self, charset: Charsets) -> bool {
        charset.contains_or_non_ascii(self)
    }
}

/// Represents the result of a lookup of a char
///
/// `CharMatchExt` is implemented on it so that you can treat it the same
/// as a char (wrt. this trait).
///
/// # Example
///
/// ```
/// use mail_chars::{Charsets, CharMatchExt};
///
/// let res = Charsets::lookup('↓');
/// assert!(!res.is_ascii());
/// assert!(!res.is(Charsets::QTextWs));
/// assert!(res.is_inkl_non_ascii(Charsets::QTextWs));
///
/// let res = Charsets::lookup('<');
/// assert!(res.is_ascii());
/// assert!(res.is(Charsets::QTextWs));
/// assert!(res.is(Charsets::CText));
/// assert!(!res.is(Charsets::AText));
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LookupResult(Option<u8>);

impl LookupResult {

    pub fn is_ascii(&self) -> bool {
        self.0.is_some()
    }

    fn lookup_contains(&self, charset: Charsets, default: bool) -> bool {
        self.0.map(|res| {
            res & (charset as u8) != 0
        }).unwrap_or(default)
    }
}

impl Seal for LookupResult {}
impl CharMatchExt for LookupResult {
    #[inline]
    fn is(self, charset: Charsets) -> bool {
        self.lookup_contains(charset, false)
    }
    #[inline]
    fn is_inkl_non_ascii(self, charset: Charsets) -> bool {
        self.lookup_contains(charset, true)
    }
}

/// reexport of all charsets (Charset::... variants) from rfc5322
pub mod rfc5322 {
    pub use super::Charsets::{QTextWs, CText, AText, DText};
}

/// reexport of all charsets (Charset::... variants) from rfc2045
pub mod rfc2045 {
    pub use super::Charsets::Token;
}

/// reexport of all charsets (Charset::... variants) from rfc6838
pub mod rfc6838 {
    pub use super::Charsets::RestrictedToken;
}


#[inline]
pub fn is_ws(ch: char) -> bool {
    ch == ' ' || ch  == '\t'
}