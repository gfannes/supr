use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct Tree {
    path: PathBuf,
    include_hidden: bool,
    include_ignored: bool,
}

impl Tree {
    pub fn new(path: PathBuf) -> Tree {
        Tree {
            path,
            include_hidden: false,
            include_ignored: false,
        }
    }
}

impl IntoIterator for &Tree {
    type Item = PathBuf;
    type IntoIter = TreeIter;
    fn into_iter(self) -> TreeIter {
        TreeIter {
            walk: ignore::WalkBuilder::new(&self.path)
                .hidden(!self.include_hidden)
                .ignore(self.include_ignored)
                .build(),
        }
    }
}

pub struct TreeIter {
    walk: ignore::Walk,
}

impl Iterator for TreeIter {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        loop {
            let entry = self.walk.next()?;
            if let Ok(entry) = entry {
                if let Some(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        return Some(entry.path().to_owned());
                    }
                }
            }
        }
    }
}

#[test]
fn create_tree() {
    let _tree = Tree::new(PathBuf::from("."));
    for p in _tree.into_iter() {
        println!("{}", p.display());
    }
}
