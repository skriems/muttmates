use std::fmt;

use crate::fields::*;

/// A `VCard` holds the new line delimited raw `String` which represents a `VCard`
#[derive(Debug, PartialEq)]
pub struct VCard<'a> {
    pub raw: &'a str,
    pub full_name: FN<'a>,
    pub email_addr: Vec<EMail<'a>>,
}

impl<'a> VCard<'a> {
    pub fn new(raw: &'a str) -> Self {
        let mut full_name: FN = FN::new("FN:UNKNOWN");
        let mut email_addr: Vec<EMail> = vec![];

        for line in raw.lines() {
            let lower = line.to_lowercase();
            if lower.starts_with("fn") {
                full_name = FN::new(line);
                continue;
            }
            if lower.starts_with("email") {
                email_addr.push(EMail::new(line));
            }
        }

        VCard {
            raw,
            full_name,
            email_addr,
        }
    }
}

impl<'a> fmt::Display for VCard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.email_addr.is_empty() {
            if self.email_addr.len() > 1 {
                let mut v: Vec<String> = vec![];
                for addr in &self.email_addr {
                    v.push(
                        vec![
                            addr.addr,
                            self.full_name.name,
                            &format!("{:?}", addr.kind),
                        ]
                        .join("\t"),
                    )
                }
                write!(f, "{}", v.join("\n"))
            } else {
                write!(
                    f,
                    "{}\t{}\t{:?}",
                    self.email_addr[0], self.full_name, self.email_addr[0].kind
                )
            }
        } else {
            write!(f, "{}", self.full_name)
        }
    }
}
