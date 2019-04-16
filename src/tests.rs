#![allow(unused_imports)]
#[cfg(test)]
extern crate muttmates;

use muttmates::fields::*;
use muttmates::VCard;

#[test]
fn test_email_with_type() {
    let email = EMail::new("EMAIL;TYPE=WORK:john@doe.example");
    assert_eq!(email.addr, "john@doe.example");
    assert_eq!(email.r#type, EMailType::Work);
}

#[test]
fn test_email_without_type() {
    let email = EMail::new("EMAIL:john@doe.example");
    assert_eq!(email.addr, "john@doe.example");
    assert_eq!(email.r#type, EMailType::Other);
}

#[test]
fn test_basic_vcard() {
    let raw = String::from(
        "BEGIN:VCARD
VERSION:3
FN:John Doe
EMAIL:john@doe.example",
    );
    let card = VCard::new(&raw);
    assert_eq!(card.full_name, FN::new("FN:John Doe"));
    assert_eq!(card.full_name.name, "John Doe".to_owned());
    assert_eq!(card.email_addr.len(), 1);
    assert_eq!(card.email_addr[0], EMail::new("EMAIL:john@doe.example"));
    assert_eq!(card.email_addr[0].r#type, EMailType::Other);
}
