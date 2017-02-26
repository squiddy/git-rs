extern crate git_rs;
extern crate clap;

use std::env;
use clap::{Arg, App};
use git_rs::{Repository, Object};

fn run(sha: &str) -> Result<(), String> {
    let cwd = env::current_dir().map_err(|err| err.to_string())?;
    let repo = Repository::open(&cwd)?;
    let object = repo.find_object(sha)?;

    match object {
        Object::Tree(t) => t.print(),
        Object::Commit(c) => c.print(),
        Object::Blob(b) => b.print(),
    }

    Ok(())
}

fn main() {
    let matches = App::new("")
        .arg(Arg::with_name("SHA")
            .help("The commit id to show information for")
            .required(true))
        .get_matches();

    let sha = matches.value_of("SHA").unwrap();

    match run(sha) {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }
}