use crate::error::Result;
use crate::pixel::{Pixel, PixelMode};
use crate::sprite::Sprite;

/// Context provides facilities to draw objects.
pub struct Context {
  screen_width: usize, // screen width in "pixels"
  screen_height: usize, // screen height in "pixels"
  current: Sprite, // current drawing target
  pixel_mode: PixelMode, // current pixel mode
  blend_factor: f32,
}

impl Context {
  /// Creates new context for the (width, height) screen size.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      screen_width: width,
      screen_height: height,
      current: Sprite::new(width, height),
      pixel_mode: PixelMode::NORMAL,
      blend_factor: 1.0,
    }
  }

  /// Returns screen width.
  #[inline]
  pub fn screen_width(&self) -> usize {
    self.screen_width
  }

  /// Returns screen height.
  #[inline]
  pub fn screen_height(&self) -> usize {
    self.screen_height
  }

  /// Returns a reference to the current drawing target.
  #[inline]
  pub fn draw_target(&self) -> &Sprite {
    &self.current
  }

  /// Returns a mutable reference to the current drawing target.
  #[inline]
  pub fn draw_target_mut(&mut self) -> &mut Sprite {
    &mut self.current
  }

  /// Returns the current pixel mode.
  #[inline]
  pub fn pixel_mode(&self) -> PixelMode {
    self.pixel_mode
  }

  /// Sets the current pixel mode to the provided value.
  #[inline]
  pub fn set_pixel_mode(&mut self, mode: PixelMode) {
    self.pixel_mode = mode;
  }

  /// Sets pixel blend factor, must be between 0 and 1.
  #[inline]
  pub fn set_pixel_blend(&mut self, blend: f32) {
    self.blend_factor = blend;
    if self.blend_factor < 0.0 {
      self.blend_factor = 0.0;
    }
    if self.blend_factor > 1.0 {
      self.blend_factor = 1.0;
    }
  }

  /// Draws pixel in the current drawing target.
  pub fn draw(&mut self, x: usize, y: usize, p: Pixel) -> Result<()> {
    match self.pixel_mode {
      PixelMode::NORMAL => self.current.set_pixel(x, y, p),
      PixelMode::MASK => {
        if p.a() == 255 {
          self.current.set_pixel(x, y, p)?;
        }
        Ok(())
      },
      PixelMode::ALPHA => {
        let d = self.current.get_pixel(x, y)?;
        let a = (p.a() as f32 / 255.0) * self.blend_factor;
        let c = 1.0 - a;
        let r = a * p.r() as f32 + c * d.r() as f32;
  			let g = a * p.g() as f32 + c * d.g() as f32;
  			let b = a * p.b() as f32 + c * d.b() as f32;
        self.current.set_pixel(x, y, Pixel::rgba(r as u8, g as u8, b as u8, a as u8))
      },
    }
  }
}
