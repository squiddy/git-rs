use std::io::{BufReader, Read, BufRead};
use std::fmt;
use std::str;
use std::fs::File;
use std::marker::Sized;

use flate2::read::ZlibDecoder;
use rustc_serialize::hex::ToHex;

trait GitObject {
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Self, String>
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
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Blob, String> {
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
    pub sha: String,
    pub tree: String,
    pub parent: Option<String>,
    pub author: String,
    pub committer: String,
    pub message: String,
}

impl Commit {
    // Debug output of the whole object
    pub fn print(&self) {
        println!("Tree {}", self.tree);
        println!("Parent {:?}", self.parent);
        println!("Author {}", self.author);
        println!("Committer {}", self.committer);
        println!("{}", self.message);
    }
}

impl GitObject for Commit {
    // Construct a commit from the given input.
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Commit, String> {
        // Read a line of the form "identifier other information" and returns
        // it as tuple ("identifier", "other information")
        fn read_line<B: BufRead>(reader: &mut B) -> (String, String) {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            let mut iter = buffer.trim().splitn(2, ' ');
            (iter.next().unwrap().to_string(), iter.next().unwrap().to_string())
        }

        // Parse tree/parent/author/committer lines.
        // FIXME This is pretty ugly.
        let (_, tree) = read_line(reader);
        let (t, data) = read_line(reader);
        if t == "parent" {
            let parent = Some(data);
            let (_, author) = read_line(reader);
            let (_, committer) = read_line(reader);

            let mut message = String::new();
            reader.read_to_string(&mut message);

            Ok(Commit {
                sha: sha.to_string(),
                tree: tree,
                parent: parent,
                author: author,
                committer: committer,
                message: message,
            })
        } else {
            let parent = None;
            let author = data;
            let (_, committer) = read_line(reader);

            let mut message = String::new();
            reader.read_to_string(&mut message);

            Ok(Commit {
                sha: sha.to_string(),
                tree: tree,
                parent: parent,
                author: author,
                committer: committer,
                message: message,
            })
        }
    }
}

impl fmt::Debug for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Commit {}", self.sha)
    }
}

pub struct TreeEntry {
    pub sha: String,
    pub filename: String,
    pub mode: String,
}

pub struct Tree {
    pub sha: String,
    pub entries: Vec<TreeEntry>,
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
    fn from_object_data<B: BufRead>(sha: &str, reader: &mut B) -> Result<Tree, String> {
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

        Ok(Tree {
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
fn read_object<R: Read>(sha: &str, input: R) -> Result<Object, String> {
    let decoder = ZlibDecoder::new(input);
    let mut reader = BufReader::new(decoder);

    let mut buffer = vec![];
    reader.read_until(0, &mut buffer);
    let header = str::from_utf8(&buffer[..buffer.len() - 1]).map_err(|err| err.to_string())?;

    match header.splitn(2, ' ').next() {
        Some("tree") => {
            let tree = Tree::from_object_data(sha, &mut reader)?;
            Ok(Object::Tree(tree))
        }
        Some("commit") => {
            let commit = Commit::from_object_data(sha, &mut reader)?;
            Ok(Object::Commit(commit))
        }
        Some("blob") => {
            let blob = Blob::from_object_data(sha, &mut reader)?;
            Ok(Object::Blob(blob))
        }
        _ => Err("unknown object type".to_string()),
    }
}

/// Returns an object parsed from the object file given by `path`.
pub fn read_object_file(sha: &str, path: &str) -> Result<Object, String> {
    let file = File::open(path).map_err(|err| err.to_string())?;
    read_object(sha, file)
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
        let data = vec![0x74, 0x72, 0x65, 0x65, 0x20, 0x30, 0x38, 0x66, 0x34, 0x38, 0x36, 0x64,
                        0x32, 0x37, 0x64, 0x36, 0x33, 0x65, 0x64, 0x37, 0x66, 0x38, 0x33, 0x38,
                        0x37, 0x36, 0x32, 0x30, 0x33, 0x61, 0x32, 0x34, 0x61, 0x61, 0x63, 0x61,
                        0x30, 0x38, 0x61, 0x32, 0x32, 0x61, 0x35, 0x63, 0x31, 0x0a, 0x70, 0x61,
                        0x72, 0x65, 0x6e, 0x74, 0x20, 0x35, 0x31, 0x33, 0x63, 0x63, 0x64, 0x32,
                        0x62, 0x36, 0x37, 0x31, 0x32, 0x34, 0x34, 0x61, 0x65, 0x65, 0x39, 0x38,
                        0x65, 0x31, 0x66, 0x30, 0x36, 0x38, 0x34, 0x66, 0x34, 0x39, 0x65, 0x65,
                        0x63, 0x38, 0x39, 0x37, 0x33, 0x63, 0x35, 0x31, 0x65, 0x0a, 0x61, 0x75,
                        0x74, 0x68, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x3c, 0x61, 0x40, 0x62, 0x2e,
                        0x64, 0x65, 0x3e, 0x20, 0x31, 0x34, 0x38, 0x38, 0x30, 0x32, 0x39, 0x38,
                        0x37, 0x34, 0x20, 0x2b, 0x30, 0x31, 0x30, 0x30, 0x0a, 0x63, 0x6f, 0x6d,
                        0x6d, 0x69, 0x74, 0x74, 0x65, 0x72, 0x20, 0x62, 0x20, 0x3c, 0x62, 0x40,
                        0x61, 0x2e, 0x64, 0x65, 0x3e, 0x20, 0x31, 0x34, 0x38, 0x38, 0x30, 0x35,
                        0x38, 0x39, 0x35, 0x39, 0x20, 0x2b, 0x30, 0x31, 0x30, 0x30, 0x0a, 0x0a,
                        0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x65,
                        0x73, 0x74, 0x0a];
        let mut reader = BufReader::new(&data[..]);

        match Commit::from_object_data("sha", &mut reader) {
            Ok(c) => {
                assert_eq!(c.sha, "sha");
                assert_eq!(c.tree, "08f486d27d63ed7f83876203a24aaca08a22a5c1");
                assert_eq!(c.parent.unwrap(),
                           "513ccd2b671244aee98e1f0684f49eec8973c51e");
                assert_eq!(c.author, "a <a@b.de> 1488029874 +0100");
                assert_eq!(c.committer, "b <b@a.de> 1488058959 +0100");
                assert_eq!(c.message, "\nThis is a test\n");
            }
            Err(err) => assert!(false, err),
        }
    }
}