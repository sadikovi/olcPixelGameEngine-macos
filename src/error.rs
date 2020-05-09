/// Error for the game engine.
#[derive(Clone, Debug)]
pub struct PGError {
  msg: String
}

impl From<&str> for PGError {
  fn from(err: &str) -> Self {
    Self { msg: err.to_owned() }
  }
}

impl From<String> for PGError {
  fn from(err: String) -> Self {
    Self { msg: err }
  }
}

pub type Result<T> = std::result::Result<T, PGError>;
