use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Default, Clone)]
pub struct Header<T> {
    header: HashMap<T, T>,
}

impl<T> Header<T> {
    pub fn from(map: HashMap<T, T>) -> Self {
        Header { header: map }
    }
}

// 实现 FromIterator 用于 collect()
impl<T: Eq + std::hash::Hash> FromIterator<(T, T)> for Header<T> {
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        Header {
            header: iter.into_iter().collect(),
        }
    }
}


// 实现 IntoIterator 以支持迭代
impl<T> IntoIterator for Header<T> {
    type Item = (T, T);
    type IntoIter = std::collections::hash_map::IntoIter<T, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.header.into_iter()
    }
}

