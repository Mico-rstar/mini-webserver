
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Binary(Vec<u8>),
    Text(String),
    None,
}

