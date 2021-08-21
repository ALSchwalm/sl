#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidAnimationSpeed,
    EmptyAnimation,
    EmptyFrame,
}

pub type Result<T> = std::result::Result<T, Error>;
