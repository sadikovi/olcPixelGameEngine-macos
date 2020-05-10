/// Pixel mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelMode {
  NORMAL,
  MASK,
  ALPHA,
}

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

  // Additional colour constants
  #[allow(non_snake_case)]
  pub fn GREY() -> Pixel { Pixel::rgb(192, 192, 192) }

  #[allow(non_snake_case)]
  pub fn DARK_GREY() -> Pixel { Pixel::rgb(128, 128, 128) }

  #[allow(non_snake_case)]
  pub fn VERY_DARK_GREY() -> Pixel { Pixel::rgb(64, 64, 64) }

  #[allow(non_snake_case)]
  pub fn RED() -> Pixel { Pixel::rgb(255, 0, 0) }

  #[allow(non_snake_case)]
  pub fn DARK_RED() -> Pixel { Pixel::rgb(128, 0, 0) }

  #[allow(non_snake_case)]
  pub fn VERY_DARK_RED() -> Pixel { Pixel::rgb(64, 0, 0) }

  #[allow(non_snake_case)]
  pub fn YELLOW() -> Pixel { Pixel::rgb(255, 255, 0) }

  #[allow(non_snake_case)]
  pub fn DARK_YELLOW() -> Pixel { Pixel::rgb(128, 128, 0) }

  #[allow(non_snake_case)]
  pub fn VERY_DARK_YELLOW() -> Pixel { Pixel::rgb(64, 64, 0) }

  #[allow(non_snake_case)]
  pub fn GREEN() -> Pixel { Pixel::rgb(0, 255, 0) }

  #[allow(non_snake_case)]
  pub fn DARK_GREEN() -> Pixel { Pixel::rgb(0, 128, 0) }

  #[allow(non_snake_case)]
  pub fn VERY_DARK_GREEN() -> Pixel { Pixel::rgb(0, 64, 0) }

  #[allow(non_snake_case)]
  pub fn CYAN() -> Pixel { Pixel::rgb(0, 255, 255) }

  #[allow(non_snake_case)]
  pub fn DARK_CYAN() -> Pixel { Pixel::rgb(0, 128, 128) }

  #[allow(non_snake_case)]
  pub fn VERY_DARK_CYAN() -> Pixel { Pixel::rgb(0, 64, 64) }

  #[allow(non_snake_case)]
  pub fn BLUE() -> Pixel { Pixel::rgb(0, 0, 255) }

  #[allow(non_snake_case)]
  pub fn DARK_BLUE() -> Pixel { Pixel::rgb(0, 0, 128) }

  #[allow(non_snake_case)]
  pub fn VERY_DARK_BLUE() -> Pixel { Pixel::rgb(0, 0, 64) }

  #[allow(non_snake_case)]
  pub fn MAGENTA() -> Pixel { Pixel::rgb(255, 0, 255) }

  #[allow(non_snake_case)]
  pub fn DARK_MAGENTA() -> Pixel { Pixel::rgb(128, 0, 128) }

  #[allow(non_snake_case)]
  pub fn VERY_DARK_MAGENTA() -> Pixel { Pixel::rgb(64, 0, 64) }

  #[allow(non_snake_case)]
  pub fn WHITE() -> Pixel { Pixel::rgb(255, 255, 255) }

  #[allow(non_snake_case)]
  pub fn BLACK() -> Pixel { Pixel::rgb(0, 0, 0) }

  #[allow(non_snake_case)]
  pub fn BLANK() -> Pixel { Pixel::rgba(0, 0, 0, 0) }
}
