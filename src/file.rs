pub struct File {
    ls: String,
    name: String,
    modified: String,
    size: String,
    perms: String,
}
impl File {

    pub fn new(ls: S) -> Self
    where S: AsRef<str> {
        Self {
            ls,
            name:
        }
    }
}
