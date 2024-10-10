use std::path::PathBuf;
use std::fs::DirEntry;

pub struct DirList {
    dir: PathBuf,
    pub items: Vec<DirItem>,
}

#[derive(Clone)]
pub struct DirItem {
    pub path: PathBuf,
    full_string: String,
    pub prefix: String,
    pub sub_path: String,
    pub name: String,
    pub class: EntryType,
}

#[derive(Clone)]
pub enum EntryType {
    Dir,
    File(FileType),
    Link,
}

#[derive(Clone)]
pub enum FileType {
    File,
}

impl DirList {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn new(path: PathBuf, uname: &str) -> DirList {
        Self {
            items: Self::list_dir(&path, &uname),
            dir: path,
        }
    }

    fn list_dir(path: &PathBuf, uname: &str) -> Vec<DirItem> {
        std::fs::read_dir(path)
            .expect("only fails if std::env::current_dir() already failed")
            .into_iter()
            .filter(|res| match res {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|entry|
                DirItem::new(entry.unwrap().path(), uname)
            ).collect()
    }
}

impl DirItem {
    pub fn new(path: PathBuf, uname: &str) -> Self {
        let full_string = path.display().to_string();
        let prefix = get_prefix(&full_string, &uname);
        let name = format!("{:?}", path.file_name());

        Self {
            class: EntryType::Dir,
            path,
            sub_path: get_subpath(&full_string, &prefix, &name, uname),
            name,
            prefix,
            full_string,
        }
    }
}


fn get_prefix(strpath: &str, uname: &str) -> String {
    if in_home(strpath, uname) {
        "~/".to_string()
    } else {
        "/".to_string()
    }
}

fn in_home(strpath: &str, uname: &str) -> bool {
    strpath.starts_with(&format!("/home/{}/", uname))
}

fn get_subpath(strpath: &str, prefix: &str, name: &str, uname: &str) -> String {
    if in_home(strpath, uname) {
        format!("~/{}{}")
    } else {

    }
}

#[test]
fn test_get_subpath() {
    assert_eq!(
        "doc/ref/",
        get_subpath("/home/rw/doc/ref/.index", "~/", ".index", "rw"),
    );
    assert_eq!(
        "etc/lvm/",
        get_subpath("/etc/lvm/lvm.conf", "/", "lvm.conf", "rw"),
    );
    assert_eq!(
        "home/redward/cs/rust/",
        get_subpath("home/redward/cs/rust/book", "/", "book", "rw"),
    );
}
