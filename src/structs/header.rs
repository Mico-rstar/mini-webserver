use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Default, Clone)]
pub struct Header {
    header: HashMap<String, String>,
}



// 实现 FromIterator 用于 collect()
impl FromIterator<(String, String)> for Header {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        Header {
            header: iter.into_iter().collect(),
        }
    }
}


// 实现 IntoIterator 以支持迭代
impl IntoIterator for Header {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.header.into_iter()
    }
}


impl Header {
    pub fn from(map: HashMap<String, String>) -> Self {
        Header { header: map }
    }


}
