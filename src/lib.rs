pub mod lookup;

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
        self.lookup(ch, false)
    }

    /// returns true if the char is part of the set of chars or not an ascii char
    ///
    /// this is mainly meant to be used in combination with rfc6532 which extends
    /// all *text grammar parts/character sets to contain any non-us-ascii character
    #[inline]
    pub fn contains_or_non_ascii(&self, ch: char) -> bool {
        self.lookup(ch, true)
    }


    fn lookup(&self, ch: char, out_of_table_value: bool) -> bool {
        let index = ch as u32;
        if index < 0x80 {
            lookup::US_ASCII_LOOKUP[index as usize] & (*self as u8) != 0
        } else {
            out_of_table_value
        }
    }
}

mod sealed{ pub trait Seal {} }
pub use self::sealed::Seal;
pub trait CharMatchExt: Seal+Copy {
    fn is(self, charset: Charsets) -> bool;
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