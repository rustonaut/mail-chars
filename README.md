mail-chars [![Crates.io](https://img.shields.io/crates/v/mail-chars.svg)](https://crates.io/crates/mail-chars) [![mail-chars](https://docs.rs/mail-chars/badge.svg)](https://docs.rs/mail-chars) [![License](https://img.shields.io/badge/License-MIT%2FApache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Build Status](https://travis-ci.org/1aim/mail-chars.svg?branch=master)](https://travis-ci.org/1aim/mail-chars)
==========
Provides lookup table based char classification for mail related grammar
parts/charset, i.e. if a given char is valid in atext, ctext, dtext, token etc.

Note that this just covers grammar parts defining sets of chars (like atext,
ctext, ...) but not contextual parts like e.g. quoted-pairs. 

```rust
extern crate mail_chars;
use mail_chars::{Charset, rfc5322, rfc2045, CharMatchExt};

fn main() {
    assert!(Charset::AText.contains('d'));
    assert!('d'.is(Charset::AText));
    assert!('d'.is(rfc5322::AText));
    
    // `rfc*::*` are just re-exports grouped by RFC.
    assert_eq!(Charset::Token, rfc2045::Token);
    
    // If we want to test for more than on char set we can use lookup.
    let res = Charset::lookup('.');

    // This has the benefit that there is an `is_ascii` method.
    assert!(res.is_ascii());
    assert!(res.is(rfc2045::Token));
    assert!(res.is(rfc5322::CTextWs));
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
