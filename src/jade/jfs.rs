use std::path::PathBuf;

pub struct Listing {
    pub name: String,
    pub class: EntryType,
    pub path: PathBuf,
}

pub enum EntryType {
    Dir,
    File(FileType),
    Link,
}

pub enum FileType {
    File,
}

impl Listing {
    pub fn new(path: PathBuf) -> Self {
        Self {
            name: path.display().to_string(),
            path,
            class: EntryType::Dir,
        }
    }
}

pub fn list_dir(path: &PathBuf) -> Vec<Listing> {
    std::fs::read_dir(path)
        .expect("only fails if std::env::current_dir() already failed")
        .into_iter()
        .filter(|res| match res {
            Ok(_) => true,
            Err(_) => false,
        })
        .map(|entry|
            Listing::new(entry.unwrap().path())
        ).collect()
}
