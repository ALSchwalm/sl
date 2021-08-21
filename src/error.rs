#[derive(Debug)]
pub enum Error {
    InvalidAnimationSpeed,
    EmptyAnimation,
    EmptyFrame,
}

pub type Result<T> = std::result::Result<T, Error>;
