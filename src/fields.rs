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
/// assert_eq!(field.parse(), "Foo Bar" );
/// ```
#[derive(Debug)]
pub struct FN<'a> {
    name: &'a str,
}

impl<'a> FN<'a> {
    pub fn new(name: &'a str) -> FN<'a> {
        FN { name }
    }

    pub fn parse(&self) -> String {
        let splits: Vec<&str> = self.name.split(":").collect();
        format!("{}", splits[1])
    }
}

impl<'a> fmt::Display for FN<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parse())
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
/// assert_eq!(email.r#type, EMailType::Work);
/// ```
#[derive(Debug)]
pub struct EMail<'a> {
    pub addr: &'a str,
    pub r#type: EMailType,
    pub pref: bool,
}

impl<'a> EMail<'a> {
    pub fn new(raw: &'a str) -> EMail {
        let splits: Vec<&str> = raw.split(":").collect();
        let (prefix, addr) = match splits.as_slice() {
            [prefix, addr] => (prefix, addr),
            _ => unreachable!(),
        };

        let mut r#type = EMailType::Other;
        let lower = prefix.to_lowercase();

        if let Some(_) = lower.find("type") {
            if lower.contains("work") {
                r#type = EMailType::Work;
            } else if lower.contains("home") {
                r#type = EMailType::Home;
            }
        }
        let pref = false;

        EMail { addr, r#type, pref }
    }

    fn parse(&self) -> String {
        format!("{}", self.addr)
    }
}

impl<'a> fmt::Display for EMail<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parse())
    }
}
