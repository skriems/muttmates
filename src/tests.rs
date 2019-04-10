#![allow(unused_imports)]
#[cfg(test)]
extern crate muttmates;

use muttmates::fields::{EMail, EMailType};

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
