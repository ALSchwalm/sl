#[derive(Debug)]
pub enum Error {
    EmptyAnimation,
}

pub type Result<T> = std::result::Result<T, Error>;
