/// Represents RGBA value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
  r: u8, // red
  g: u8, // green
  b: u8, // blue
  a: u8 // alpha, 0 - transparent, 255 - opaque
}

impl Pixel {
  /// Returns RGBA value.
  #[inline]
  pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  /// Returns RGBA with alpha as 255.
  #[inline]
  pub fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self::rgba(r, g, b, 255)
  }

  /// Red component.
  #[inline]
  pub fn r(&self) -> u8 {
    self.r
  }

  /// Green component.
  #[inline]
  pub fn g(&self) -> u8 {
    self.g
  }

  /// Blue component.
  #[inline]
  pub fn b(&self) -> u8 {
    self.b
  }

  /// Alpha component.
  #[inline]
  pub fn a(&self) -> u8 {
    self.a
  }
}
