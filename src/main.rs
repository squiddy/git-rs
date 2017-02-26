extern crate git_rs;
extern crate clap;
extern crate ansi_term;

use std::env;
use clap::{Arg, App, SubCommand, ArgMatches};
use ansi_term::Colour::{Yellow, Red};
use git_rs::{Object, Repository};

type CliResult = Result<(), String>;

fn command_cat_file(matches: &ArgMatches) -> CliResult {
    let sha = matches.value_of("SHA").unwrap();
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

fn command_log(matches: &ArgMatches) -> CliResult {
    let sha = matches.value_of("SHA").unwrap();
    let cwd = env::current_dir().map_err(|err| err.to_string())?;
    let repo = Repository::open(&cwd)?;

    for commit in repo.log(sha)? {
        println!("{}", Yellow.paint(format!("commit {}", commit.sha)));
        println!("Author: {}", commit.author);

        for line in commit.message.lines() {
            println!("    {}", line);
        }

        println!("");
    }

    Ok(())
}

fn main() {
    let matches = App::new("git")
        .subcommand(SubCommand::with_name("log").arg(Arg::with_name("SHA")
            .help("The commit id from which to start the history log")
            .required(true)))
        .subcommand(SubCommand::with_name("cat-file").arg(Arg::with_name("SHA")
            .help("The commit id to show information for")
            .required(true)))
        .get_matches();

    let result = match matches.subcommand() {
        ("log", Some(m)) => command_log(m),
        ("cat-file", Some(m)) => command_cat_file(m),
        _ => {
            println!("{}", matches.usage());
            Ok(())
        }
    };

    match result {
        Ok(()) => (),
        Err(err) => println!("{}", Red.paint(format!("Error: {}", err))),
    }
}