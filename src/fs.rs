use std::path::PathBuf;

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
            root: self.path.clone(),
            walk: ignore::WalkBuilder::new(&self.path)
                .hidden(!self.include_hidden)
                .ignore(self.include_ignored)
                .build(),
        }
    }
}

pub struct TreeIter {
    root: PathBuf,
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
                        if let Ok(rel) = entry.path().strip_prefix(&self.root) {
                            return Some(rel.to_owned());
                        } else {
                            eprintln!(
                                "Could not strip root '{}' from '{}'",
                                self.root.display(),
                                entry.path().display()
                            );
                            return None;
                        }
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
