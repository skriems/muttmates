/*
 * TODO: add support for:
 * "SOURCE" / "KIND" / "FN" / "N" / "NICKNAME"
 * "PHOTO" / "BDAY" / "ANNIVERSARY" / "GENDER" / "ADR" / "TEL"
 * "EMAIL" / "IMPP" / "LANG" / "TZ" / "GEO" / "TITLE" / "ROLE"
 * "LOGO" / "ORG" / "MEMBER" / "RELATED" / "CATEGORIES"
 * "NOTE" / "PRODID" / "REV" / "SOUND" / "UID" / "CLIENTPIDMAP"
 * "URL" / "KEY" / "FBURL" / "CALADRURI" / "CALURI" / "XML"
 * iana-token / x-name
*/

use std::fmt;

/// The `FN` field of a `VCard` is a required field
/// # Example
/// ```
/// use muttmates::fields::FN;
///
/// let field = FN::new("FN:Foo Bar");
/// assert_eq!(field.name, "Foo Bar" );
/// ```
#[derive(Debug, PartialEq)]
pub struct FN<'a> {
    pub name: &'a str,
}

impl<'a> FN<'a> {
    pub fn new(raw: &'a str) -> FN<'a> {
        let splits: Vec<&str> = raw.split(':').collect();
        FN { name: splits[1] }
    }
}

impl<'a> fmt::Display for FN<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// An Enum for various EMail types
/// TODO: please elaborate...
#[derive(Debug, PartialEq)]
pub enum EMailType {
    Home,
    Work,
    Other,
}

/// The `EMAIL` field
/// # Example
/// ```
/// use muttmates::fields::{EMail, EMailType};
///
/// let email = EMail::new("EMAIL;TYPE=WORK:john@doe.example");
/// assert_eq!(email.addr, "john@doe.example");
/// assert_eq!(email.kind, EMailType::Work);
/// ```
#[derive(Debug, PartialEq)]
pub struct EMail<'a> {
    pub addr: &'a str,
    pub kind: EMailType,
    pub pref: bool,
}

impl<'a> EMail<'a> {
    pub fn new(raw: &'a str) -> EMail {
        let splits: Vec<&str> = raw.split(":").collect();
        let (prefix, addr) = match splits.as_slice() {
            [prefix, addr] => (prefix, addr),
            _ => unreachable!(),
        };

        let mut kind = EMailType::Other;
        let lower = prefix.to_lowercase();

        if lower.find("type").is_some() {
            if lower.contains("work") {
                kind = EMailType::Work;
            } else if lower.contains("home") {
                kind = EMailType::Home;
            }
        }
        let pref = false;

        EMail { addr, kind, pref }
    }

    fn parse(&self) -> String {
        self.addr.to_string()
    }
}

impl<'a> fmt::Display for EMail<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parse())
    }
}
