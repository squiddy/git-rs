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

#[test]
fn find_commit_object_first_in_history() {
    let repo = Repository::open(Path::new(".git")).unwrap();

    match repo.find_object("513ccd2b671244aee98e1f0684f49eec8973c51e") {
        Object::Commit(c) => {
            assert_eq!(c.tree, "4ee92b6df668a7531af74d4c2e6bbafce7b55e3b");
            assert_eq!(c.parent, None);
            assert_eq!(c.author, "Reiner Gerecke <me@reinergerecke.de> 1488024725 +0100");
            assert_eq!(c.committer, "Reiner Gerecke <me@reinergerecke.de> 1488024725 +0100");
            assert_eq!(c.message, "\nInitial commit");
        },
        _ => assert!(false, "Object not found or unexpected type")
    }
}

#[test]
fn find_commit_object() {
    let repo = Repository::open(Path::new(".git")).unwrap();

    match repo.find_object("da7a94937cb5bf2635ad143412a3bbe5ab67ff22") {
        Object::Commit(c) => {
            assert_eq!(c.tree, "0c0ae74bbf7f93c18eb2a266ca1d2f5fde3e7260");
            assert!(c.parent.is_some());
            assert_eq!(c.parent.unwrap(), "e4a9c098f6d1bef856bfeab26e993401c4c05eb2");
            assert_eq!(c.author, "Reiner Gerecke <me@reinergerecke.de> 1488035936 +0100");
            assert_eq!(c.committer, "Reiner Gerecke <me@reinergerecke.de> 1488035936 +0100");
            assert_eq!(c.message, "\nAdd README\n");
        },
        _ => assert!(false, "Object not found or unexpected type")
    }
}