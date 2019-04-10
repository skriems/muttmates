[![Build Status](https://travis-ci.org/skriems/muttmates.svg?branch=master)](https://travis-ci.org/skriems/muttmates)
![crates.io](https://img.shields.io/crates/v/muttmates.svg)

muttmates
=====

> A simple tool to retrieve email addresses in a mutt compatible format

The question which drove this project was simple:
_How do I get my contacts into mutt?_

Initially I wanted to write a library around
[RFC6350](https://tools.ietf.org/html/rfc6350) but just now I have found that
[others already did](https://crates.io/crates/vcard).

Anyhow, I was looking for a very simple solution and the available options did not
convince me really, so I exported my contacts from my nextcloud and started
this project...

Tbh, now that I've seen that there already exist a bunch of crates related to
that RFC I have no idea what to do with this project. It still would be nice to
have a tool for managing your contacts and syncing it via i.e. carddav... We'll
see. At least I get my contacts in mutt now _blazingly fast_ ;)

## Installation

To install `muttmates` use cargo and add `~/.cargo/bin` to your PATH:
```
cargo install muttmates
```

## Usage

By default `muttmates` looks for vcf files in `~/.vcards`. So either move your
files there or specify a custom location or file with the `-c` option.

The current version is really meant to be used for querying email addresses
with the `query` argument. It will output the contacts in a tab seperated
fashion that mutt demands:
```
john.doe@example.com    John Doe    Home
<EMAIL>                <FN>        <EMAIL;TYPE>
```

If you don't `query` for an address the output is rather awkward and
unfinished.  This is going to be fixed soon, though I don't know when I find
time to support more fields.

### Configure Mutt

Using the default folder for vcf files you only have to setup the
`query_command` in the `muttrc` file like so:
```
set query_command="muttmates %s"
```
Or optionally provide the path to your vcf file or folder containing such:
```
set query_command="muttmates -c path/to/contacts %s"
```
