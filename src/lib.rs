//! A DNS library for Rust.
//!
//! This crate provides a wide range of modules related to the Domain Name
//! System. Currently, these are:
//!
//! * fundamental types, traits, and implementations for dealing with DNS
//!   data through the modules [bits] and [iana],
//! * parsing of master file data (aka zonefiles) in [master],
//! * data and master file access for various resource record types in
//!   [rdata],
//! * an asynchronous stub resolver implementation for querying the DNS
//!   in [resolv].
//!
//! [bits]: bits/index.html
//! [iana]: iana/index.html
//! [master]: master/index.html
//! [rdata]: rdata/index.html
//! [resolv]: resolv/index.html
#![allow(unknown_lints)] // hide clippy-related #allows on stable. 

extern crate base64;
extern crate bytes;
extern crate chrono;
extern crate failure;
#[macro_use] extern crate failure_derive;
/*#[macro_use]*/ extern crate futures;
extern crate rand;
/*#[macro_use]*/ extern crate tokio;

pub mod bits;
pub mod iana;
pub mod master;
pub mod rdata;
pub mod resolv;
pub mod utils;
