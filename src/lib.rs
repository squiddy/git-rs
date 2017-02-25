//! A implementation of git in rust
//!
//! Note: This is meant as a learning experience, and so will never be fully featured.

extern crate flate2;
extern crate rustc_serialize;

mod objects;
mod repository;

pub use objects::{Object, read_object_file};
pub use repository::{find_git_directory, Repository};