mail-chars [![Crates.io](https://img.shields.io/crates/v/mail-chars.svg)](https://crates.io/crates/quoted-string) [![quoted-string](https://docs.rs/quoted-string/badge.svg)](https://docs.rs/quoted-string) [![License](https://img.shields.io/badge/License-MIT%2FApache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Build Status](https://travis-ci.org/1aim/quoted-string.svg?branch=master)](https://travis-ci.org/1aim/mail-chars)
=============

provides char classification for mail related grammar parts / charset

For example if a given char belongs characters valid in atext, ctext, dtext, token etc.

```rust
extern crate mail_chars;
use mail_chars::{Charsets, rfc5322, rfc2045, CharMatchExt};

fn main() {
    assert!(Charsets::AText.contains('d'));
    assert!('d'.is(Charsets::AText));
    assert!('d'.is(rfc5322::AText));
    
    // rfc*::* are just reexports grouped by rfc
    assert_eq!(Charsets::Token, rfc2045::Token);
    
    // if we want to test for more than on char set we can use lookup
    let res = Charsets::lookup('.');
    // has the benefit that there is a is_ascii method 
    assert!(res.is_ascii());
    assert!(res.is(rfc2045::Token));
    assert!(res.is(rfc5322::CText));
    assert!(!res.is(rfc5322::AText));
}
```


License
=======
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
