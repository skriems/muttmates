#![allow(unused_imports)]
#[cfg(test)]
use muttmates::fields::*;
use muttmates::VCard;

#[test]
fn test_email_with_type() {
    let email = EMail::new("EMAIL;TYPE=WORK:john@doe.example");
    assert_eq!(email.addr, "john@doe.example");
    assert_eq!(email.kind, EMailKind::Work);
}

#[test]
fn test_email_with_type_and_whitespace() {
    let email = EMail::new("EMAIL; TYPE= WORK: john@doe.example");
    assert_eq!(email.addr, "john@doe.example");
    assert_eq!(email.kind, EMailKind::Work);
}

#[test]
fn test_email_without_type() {
    let email = EMail::new("EMAIL:john@doe.example");
    assert_eq!(email.addr, "john@doe.example");
    assert_eq!(email.kind, EMailKind::Other);
}

#[test]
fn test_vcard_basic() {
    let raw = String::from(
        "BEGIN:VCARD
VERSION:3
FN:John Doe
EMAIL:john@doe.example
EMAIL;TYPE=WORK:john@work.example
EMAIL;TYPE=HOME:john@home.example",
    );
    let card = VCard::new(&raw);
    assert_eq!(card.full_name, FN::new("FN:John Doe"));
    assert_eq!(card.full_name.name, "John Doe".to_owned());
    assert_eq!(card.email_addr.len(), 3);
    assert_eq!(card.email_addr[0], EMail::new("EMAIL:john@doe.example"));
    assert_eq!(card.email_addr[0].kind, EMailKind::Other);
    assert_eq!(
        card.email_addr[1],
        EMail::new("EMAIL;TYPE=WORK:john@work.example")
    );
    assert_eq!(card.email_addr[1].kind, EMailKind::Work);
    assert_eq!(
        card.email_addr[2],
        EMail::new("EMAIL;TYPE=HOME:john@home.example")
    );
    assert_eq!(card.email_addr[2].kind, EMailKind::Home);
}

#[test]
fn test_vcard_display_without_address() {
    let raw = String::from(
        "BEGIN:VCARD
VERSION:3
FN:John Doe",
    );
    let card = VCard::new(&raw);
    let s = format!("{}", card);
    assert_eq!(s, String::from("John Doe"));
}

#[test]
fn test_vcard_display_single_address() {
    let raw = String::from(
        "BEGIN:VCARD
VERSION:3
FN:John Doe
EMAIL:john@doe.example",
    );
    let card = VCard::new(&raw);
    let s = format!("{}", card);
    assert_eq!(s, String::from("john@doe.example\tJohn Doe\tOther"));
}

#[test]
fn test_vcard_display_two_addresses() {
    let raw = String::from(
        "BEGIN:VCARD
VERSION:3
FN:John Doe
EMAIL:john@doe.example
EMAIL:john@doe.example.two",
    );
    let card = VCard::new(&raw);
    let s = format!("{}", card);
    assert_eq!(
        s,
        String::from("john@doe.example\tJohn Doe\tOther\njohn@doe.example.two\tJohn Doe\tOther")
    );
}
