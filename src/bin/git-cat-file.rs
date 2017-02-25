extern crate git_rs;
extern crate getopts;

use std::env;
use getopts::Options;
use git_rs::{Repository, Object};

fn print_usage(opts: Options) {
    let brief = format!("Usage: git-cat-file SHA");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "");
    opts.optopt("s", "sha", "", "SHA");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("Error: {}", f.to_string());
            print_usage(opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let sha = env::args().skip(1).next().expect("Expected sha");
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