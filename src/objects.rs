use std::io::{BufReader, Read, BufRead};
use std::fmt;
use std::str;
use std::fs::File;
use std::marker::Sized;

use flate2::read::ZlibDecoder;
use rustc_serialize::hex::ToHex;

trait GitObject {
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Self, &'static str>
        where Self: Sized;
}

pub enum Object {
    Tree(Tree),
    Commit(Commit),
    Blob(Blob),
}

pub struct Blob {
    sha: String,
    data: Vec<u8>,
}

impl Blob {
    // Debug output of the whole object
    pub fn print(&self) {
        println!("... binary ...");
    }
}

impl GitObject for Blob {
    // Construct a blob from the given input.
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Blob, &'static str> {
        let mut data = vec![];
        reader.read_to_end(&mut data);

        Ok(Blob {
            sha: sha.to_string(),
            data: data,
        })
    }
}

impl fmt::Debug for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blob {}", self.sha)
    }
}

pub struct Commit {
    sha: String,
    message: String,
}

impl Commit {
    // Debug output of the whole object
    pub fn print(&self) {
        println!("{}", self.message);
    }
}

impl GitObject for Commit {
    // Construct a commit from the given input.
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Commit, &'static str> {
        let mut message = String::new();
        reader.read_to_string(&mut message);

        Ok(Commit {
            sha: sha.to_string(),
            message: message,
        })
    }
}

impl fmt::Debug for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Commit {}", self.sha)
    }
}

struct TreeEntry {
    sha: String,
    filename: String,
    mode: String,
}

pub struct Tree {
    sha: String,
    entries: Vec<TreeEntry>,
}

impl Tree {
    // Debug output of the whole object
    pub fn print(&self) {
        for entry in &self.entries {
            println!("{:0>6} {:30} {}", entry.mode, entry.filename, entry.sha);
        }
    }
}

impl GitObject for Tree {
    // Construct a tree from the given input.
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Tree, &'static str> {
        let mut buffer = vec![];
        let mut entries = vec![];

        loop {
            buffer.clear();
            match reader.read_until(0, &mut buffer) {
                Ok(n) if n > 0 => (),
                _ => break,
            }
            let line = String::from_utf8(buffer[..buffer.len() - 1].to_vec()).unwrap();
            let matches: Vec<&str> = line.splitn(2, ' ').collect();

            let mut sha = [0; 20];
            reader.read_exact(&mut sha);

            entries.push(TreeEntry {
                sha: sha.to_hex(),
                mode: matches[0].to_string(),
                filename: matches[1].to_string(),
            })
        }

        Ok::<Tree, &'static str>(Tree {
            sha: sha.to_string(),
            entries: entries,
        })
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree {}", self.sha)
    }
}

// Returns an object parsed from the given input.
//
// Will delegate to specific implementations according to the type in the
// header.
fn read_object<R: Read>(sha: &str, input: R) -> Object {
    let decoder = ZlibDecoder::new(input);
    let mut reader = BufReader::new(decoder);

    {
        let mut buffer = vec![];
        reader.read_until(0, &mut buffer);
        let header = str::from_utf8(&buffer[..buffer.len() - 1]).unwrap();
        match header.splitn(2, ' ').next() {
            Some("tree") => Object::Tree(Tree::from_object_data(sha, &mut reader).unwrap()),
            Some("commit") => Object::Commit(Commit::from_object_data(sha, &mut reader).unwrap()),
            Some("blob") => Object::Blob(Blob::from_object_data(sha, &mut reader).unwrap()),
            _ => panic!("error"),
        }
    }
}

/// Returns an object parsed from the object file given by `path`.
pub fn read_object_file(sha: &str, path: &str) -> Object {
    match File::open(path) {
        Ok(f) => read_object(sha, f),
        Err(err) => panic!(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_blob() {
        let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x0a];
        let mut reader = BufReader::new(&data[..]);

        match Blob::from_object_data("sha", &mut reader) {
            Ok(b) => {
                assert_eq!(b.sha, "sha");
                assert_eq!(b.data,
                           [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
                            0x0a]);
            }
            Err(err) => assert!(false, err),
        }
    }

    #[test]
    fn parse_tree() {
        let data = vec![0x31, 0x30, 0x30, 0x36, 0x34, 0x34, 0x20, 0x61, 0x00, 0x3f, 0xa0, 0xd4,
                        0xb9, 0x82, 0x89, 0xa9, 0x5a, 0x7c, 0xd3, 0xa4, 0x5c, 0x95, 0x45, 0xe6,
                        0x22, 0x71, 0x8f, 0x8d, 0x2b, 0x31, 0x30, 0x30, 0x36, 0x34, 0x34, 0x20,
                        0x62, 0x00, 0xe6, 0x9d, 0xe2, 0x9b, 0xb2, 0xd1, 0xd6, 0x43, 0x4b, 0x8b,
                        0x29, 0xae, 0x77, 0x5a, 0xd8, 0xc2, 0xe4, 0x8c, 0x53, 0x91];
        let mut reader = BufReader::new(&data[..]);

        match Tree::from_object_data("sha", &mut reader) {
            Ok(t) => {
                assert_eq!(t.sha, "sha");
                assert_eq!(t.entries.len(), 2);
                assert_eq!(t.entries[0].mode, "100644");
                assert_eq!(t.entries[0].filename, "a");
                assert_eq!(t.entries[0].sha, "3fa0d4b98289a95a7cd3a45c9545e622718f8d2b");
                assert_eq!(t.entries[1].mode, "100644");
                assert_eq!(t.entries[1].filename, "b");
                assert_eq!(t.entries[1].sha, "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");
            }
            Err(err) => assert!(false, err),
        }
    }

    #[test]
    fn parse_commit() {
        let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x0a];
        let mut reader = BufReader::new(&data[..]);

        match Commit::from_object_data("sha", &mut reader) {
            Ok(c) => {
                assert_eq!(c.sha, "sha");
                assert_eq!(c.message, "Hello World\n");
            }
            Err(err) => assert!(false, err),
        }
    }
}