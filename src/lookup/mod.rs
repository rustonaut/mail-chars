// ctext    = VCHAR - '(', ')',  '\\'
// dtext    = VCHAR - '[', '\\', ']'
// atext    = VCHAR - specials
// specials = '(' | ')' | '<' | '>' | '[' | ']' | ':' | ';' | '@' | '\\'| ',' | '.' | '"'

// ctl          = (!VCHAR) -  ' '
// ftext    = VCHAR - ':'
// WS       = ' ', '\t'
// Not Implemented as token_rfc2047 is too content dependent
//mod rfc2047 {
//    // token    = US-ASCII-CHAR - ' ', CTL, especial
//    // especial = '(' | ')' | '<' | '>' | '@' | ',' | ';' | ':' | '"' | '/'| '[' | ']' | '?' | '.' | '='
//    // encoded-text = VCHAR - '?', ' '
//    //                  in comment - '(', ')', '"'
//    //                  in phrase === ALPHA/DIGIT/ "!", "*", "+", "-", "/", "=", and "_"
//    //                  in ...
//
//}

/// ctext (rfc5322)
pub const CT: u8 = 0b00000001;
/// dtext (rfc5322)
pub const DT: u8 = 0b00000010;
/// atext (rfc5322)
pub const AT: u8 = 0b00000100;
/// qtext + ws (rfc5322) (or the content of quoted strings without quoted-pairs)
pub const QC: u8 = 0b00001000;
/// restricted-name-char (rfc6838)  (which is like a restricted token hence RT)
pub const RT: u8 = 0b00010000;
/// token (rfc5322)
pub const TO: u8 = 0b00100000;


pub static US_ASCII_LOOKUP: &[u8] = &[
    //0x00
    //0/8              1/9                2/A                3/B                4/C                5/D                6/E                7/F
    0,                 0,                 0,                 0,                 0,                 0,                 0,                 0,
    0,                 QC,                0,                 0,                 0,                 0,                 0,                 0,
    //0x10                                                                                                                               
    0,                 0,                 0,                 0,                 0,                 0,                 0,                 0,
    0,                 0,                 0,                 0,                 0,                 0,                 0,                 0,
    //0x20                                                                                                                               
    QC,                CT|DT|AT|QC|RT|TO, CT|DT,             CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|TO,    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|TO,
    DT|QC,             DT|QC,             CT|DT|AT|QC|TO,    CT|DT|AT|QC|RT|TO, CT|DT|QC,          CT|DT|AT|QC|RT|TO, CT|DT|QC|RT|TO,    CT|DT|AT|QC,
    //0x30                                                                                                                               
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|QC,          CT|DT|QC,          CT|DT|QC,          CT|DT|AT|QC,       CT|DT|QC,          CT|DT|AT|QC,
    //0x40                                                                                                                               
    CT|DT|QC,          CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    //0x50                                                                                                                               
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|QC,             0,                 CT|QC,             CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    //0x60                                                                                                                               
    CT|DT|AT|QC|TO,    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    //0x70                                                                                                                               
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO,
    CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|RT|TO, CT|DT|AT|QC|TO,    CT|DT|AT|QC|TO,    CT|DT|AT|QC|TO,    CT|DT|AT|QC|TO,    0,
    //0x80
];

#[cfg(test)]
mod test {
    use super::*;

    fn is_qtext(ch: char) -> bool {
        match ch {
            //not ' ' [d:32]
            '!' |
            //not '"' [d:34]
            '#'...'[' |
            //not '\\' [d:92]
            ']'...'~' => true,
            _ => false
        }
    }

    fn is_tspecial(ch: char) -> bool {
        match ch {
            '(' | ')' | '<' | '>'  | '@' |
            ',' | ';' | ':' | '\\' | '"' |
            '/' | '[' | ']' | '?'  | '=' => true,
            _ => false
        }
    }

    fn is_restricted_char(ch: char) -> bool {
        match ch {
            '0'...'9' | 'A'...'Z' |
            'a'...'z' | '!' | '#' |
            '$' | '&' | '-' | '^' |
            '_' | '+' | '.'  => true,
            _ => false
        }
    }

    fn is_token(ch: char) -> bool {
        let bch = ch as u32;
        0x20 < bch && bch < 0x7f && !is_tspecial(ch)
    }

    fn is_qc(ch: char) -> bool {
        is_qtext(ch) || ch == ' ' || ch == '\t'
    }

    fn is_atext(ch: char ) -> bool {
        let bch = ch as u32;
        32 < bch && bch < 0x7f && match ch {
            '(' | ')' |
            '<' | '>' |
            '[' | ']' |
            ':' | ';' |
            '@' | '\\'|
            ',' | '.' |
            '"' => false,
            _ => true
        }
    }

    fn is_dtext( ch: char ) -> bool {
        match ch as u32 {
            33...90 |
            94...126 => true,
            _ => false
        }
    }

    fn is_ctext( ch: char ) -> bool {
        match ch {
            '!'...'\'' |
            '*'...'[' |
            ']'...'~' => true,
            // obs-ctext
            _ => false
        }
    }

    macro_rules! cmp {
        ($inp:expr, $res:expr, $mask:expr, $func:ident) => ({
            use std::str;
            assert!($inp < 0x80);
            let tmp = &[$inp as u8];
            let str = str::from_utf8(tmp).unwrap();
            let ch = str.chars().next().unwrap();
            let expected = $func(ch);
            assert!(($res & $mask != 0) == expected, "char 0x{:>02x} ({:?}) failed {} = {} [{:b}]",
                $inp, ch, stringify!($func), expected, $res);
        });
    }
    #[test]
    fn validate_lookup_table() {
        assert_eq!(US_ASCII_LOOKUP.len(), 128);
        for idx in 0..0x80 {
            let res = US_ASCII_LOOKUP[idx];

            cmp!(idx, res, QC, is_qc);
            cmp!(idx, res, RT, is_restricted_char);
            cmp!(idx, res, TO, is_token);
            cmp!(idx, res, AT, is_atext);
            cmp!(idx, res, DT, is_dtext);
            cmp!(idx, res, CT, is_ctext);
        }
    }
}
