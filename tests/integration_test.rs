extern crate git_rs;

use std::path::Path;
use git_rs::{Repository, Object};

#[test]
fn open_repository() {
    assert!(Repository::open(Path::new(".git")).is_ok());
}

#[test]
fn find_tree_object() {
    let repo = Repository::open(Path::new(".git")).unwrap();

    match repo.find_object("4ee92b6df668a7531af74d4c2e6bbafce7b55e3b") {
        Object::Tree(t) => {
            assert_eq!(t.entries.len(), 1);
            assert_eq!(t.entries[0].filename, "LICENSE");
            assert_eq!(t.entries[0].mode, "100644");
            assert_eq!(t.entries[0].sha, "459815d269682b8707bdf3ba103cdd6970c28853");
        },
        _ => assert!(false, "Object not found or unexpected type")
    }
}