extern crate git_rs;
extern crate clap;

use std::env;
use clap::{Arg, App};
use git_rs::{Repository};

fn run(sha: &str) -> Result<(), String> {
    let cwd = env::current_dir().map_err(|err| err.to_string())?;
    let repo = Repository::open(&cwd)?;

    for commit in repo.log(sha)? {
        println!("commit {}", commit.sha);
        println!("Author: {}", commit.author);

        for line in commit.message.lines() {
            println!("    {}", line);
        }

        println!("");
    }

    Ok(())
}

fn main() {
    let matches = App::new("")
        .arg(Arg::with_name("SHA")
            .help("The commit id from which to start the history log")
            .required(true))
        .get_matches();

    let sha = matches.value_of("SHA").unwrap();

    match run(sha) {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err)
    }
}