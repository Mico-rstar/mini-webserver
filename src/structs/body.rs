

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Binary(Vec<u8>),
    Text(String),
    None,
}

impl Body {
    pub fn len(&self) -> usize {
        match self {
            Self::Binary(data) => data.len(),
            Self::Text(data) => data.len(),
            Self::None => 0,
        }
    }
}

