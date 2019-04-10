//! # muttmates
//!
//! # What is it for?
//! It's a simple tool for retrieving email addresses in a mutt compatible format from
//! local vcf files. Nothing more, nothing less.
//!
//! ## What is it doing?
//! `muttmates` reads all your vcf files and given you have provided a `query` it will only print out
//! matched contacts in a form, that mutt is happy with. `muttmates` expects to find vcf files in your
//! home directory at `~/.vcards` but you can also pass a single file or directory with the `-c`
//! option.
//!
//! ## How do I configure mutt?
//! Given that you use the default directory for your vcf files, add the following line to your
//! `~/.muttrc` file
//!
//! ```bash
//! set query_command="muttmates %s"
//! ```

pub mod card;
pub mod fields;

// reexport card module
pub use card::*;

use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

/// Reads VCards of one or multiple files and returns a vector of splits
pub fn read_cards(source: &str) -> io::Result<Vec<String>> {
    let path = Path::new(&source);
    let mut raw = String::new();

    if path.is_dir() {
        for file in vcf_files(&path)? {
            if let Some(s) = file.path().to_str() {
                let content = read_file(s)?;
                raw.push_str(&content);
            }
        }
    } else if path.is_file() {
        raw.push_str(&read_file(source)?);
    }

    let splits = raw
        .split("BEGIN:VCARD")
        .filter(|x| x.len() > 0)
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<String>>();
    Ok(splits)
}

/// Read the content of a file into a `String`
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    fs::File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

/// Returns a vector of vcf files in a directory to which the user has permissions
pub fn vcf_files(path: &Path) -> io::Result<Vec<fs::DirEntry>> {
    // TODO return Iterator
    let files = fs::read_dir(path)?
        .filter_map(|f| f.ok())
        .filter_map(|f| is_vcf(f))
        .collect::<Vec<fs::DirEntry>>();
    Ok(files)
}

/// Returns an `Option<fs::DirEntry>` if the lowercase extension corresponds to "vcf".
fn is_vcf(file: fs::DirEntry) -> Option<fs::DirEntry> {
    if let Some(extension) = file.path().extension() {
        if let Some(ext) = extension.to_str() {
            let lower_ext = String::from(ext).to_lowercase();
            if lower_ext == "vcf" {
                return Some(file);
            }
        }
    }
    return None;
}
