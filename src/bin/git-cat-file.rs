extern crate git_rs;
extern crate clap;

use std::env;
use clap::{Arg, App};
use git_rs::{Repository, Object};

fn main() {
    let matches = App::new("")
        .arg(Arg::with_name("SHA")
            .help("The commit id to show information for")
            .required(true))
        .get_matches();

    let sha = matches.value_of("SHA").unwrap();
    let cwd = env::current_dir().expect("Can't get current directory");

    if let Ok(repo) = Repository::open(&cwd) {
        match repo.find_object(&sha) {
            Object::Tree(t) => t.print(),
            Object::Commit(c) => c.print(),
            Object::Blob(b) => b.print(),
        }
    } else {
        panic!("Couldn't find git directory");
    }
}