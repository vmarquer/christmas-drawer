#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Member {
    pub name: String,
    pub gifts: Vec<String>,
}