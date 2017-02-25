use std::path::{Path, PathBuf};

/// Starting from `path` walks up the tree to find a git directory and returns
/// it if found.
pub fn find_git_directory(path: &Path) -> Option<PathBuf> {
    let mut dir = path.to_path_buf();

    while !dir.as_path().join(".git").is_dir() {
        if !dir.pop() {
            return None;
        }
    }

    Some(dir.join(".git"))
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;
    use std::fs;
    use self::tempdir::TempDir;

    #[test]
    fn find_git_directory_exists_same_directory() {
        let temp_dir = TempDir::new("test").unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        assert_eq!(find_git_directory(&git_dir), Some(git_dir));
    }

    #[test]
    fn find_git_directory_not_found() {
        let temp_dir = TempDir::new("test").unwrap();
        assert_eq!(find_git_directory(&temp_dir.path()), None);
    }

    #[test]
    fn find_git_directory_sub_directory() {
        let temp_dir = TempDir::new("test").unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let cwd = temp_dir.path().join("a/b/c");
        fs::create_dir_all(&cwd).unwrap();

        assert_eq!(find_git_directory(&cwd), Some(git_dir));
    }
}