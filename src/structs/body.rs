
#[derive(Debug)]
pub enum Body {
    Binary(Vec<u8>),
    Text(String),
}

