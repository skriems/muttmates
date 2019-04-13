mod cli;
mod tests;

use std::io;
use std::process;

use dirs::home_dir;
use structopt::StructOpt;

use muttmates::{read_cards, VCard};

fn run(opt: cli::Opt) -> Result<(), io::Error> {
    let mut source = String::new();
    if let Some(s) = opt.contacts {
        source.push_str(s.to_str().unwrap());
    } else if let Some(dir) = home_dir() {
        if let Some(dir_string) = dir.to_str() {
            source.push_str(dir_string);
            source.push_str("/.vcards");
        }
    }

    if let Some(query) = &opt.query {
        // we need to print a new line first. Otherwise mutt prints the first match into the
        // status line ¯\_(ツ)_/¯
        println!();
        for raw in read_cards(&source)? {
            if raw.contains(query) && raw.to_lowercase().contains("email") {
                println!("{}", VCard::new(&raw));
            }
        }
    } else {
        for raw in read_cards(&source)? {
            if raw.to_lowercase().contains("email") {
                println!("{}", VCard::new(&raw));
            }
        }
    }

    Ok(())
}

fn main() {
    let opt = cli::Opt::from_args();

    if let Err(e) = run(opt) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
