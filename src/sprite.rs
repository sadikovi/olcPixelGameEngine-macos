use sdl2::pixels::PixelFormatEnum;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

use crate::error::{PGError, Result};
use crate::pixel::Pixel;

/// Sample mode for a sprite.
#[derive(Clone, Copy, Debug)]
pub enum Mode {
  NORMAL,
  PERIODIC,
}

/// Flip for a sprite.
#[derive(Clone, Copy, Debug)]
pub enum Flip {
  NONE,
  HORIZ,
  VERT,
}

/// Sprite stores data in RGBA8888.
/// Pitch is always width * 4
#[derive(Clone, Debug)]
pub struct Sprite {
  width: usize, // in pixels
  height: usize, // in pixels
  mode: Mode,
  data: Vec<u8> // raw data, each pixel is 4 bytes
}

impl Sprite {
  /// Creates new empty sprite of width and height.
  pub fn new(width: usize, height: usize) -> Self {
    let data: Vec<u8> = vec![255; width * 4 * height];
    let mode = Mode::NORMAL;
    Self { width, height, mode, data }
  }

  /// Loads sprite from an image
  pub fn from_image(path: &str) -> Result<Self> {
    let image = Surface::from_file(path)?;
    let width = image.rect().width() as usize;
    let height = image.rect().height() as usize;
    let pitch = image.pitch() as usize;

    let mut data: Vec<u8> = Vec::with_capacity(pitch * height);

    image.with_lock(|arr: &[u8]| {
      match image.pixel_format_enum() {
        PixelFormatEnum::RGBA8888 => {
          data.extend_from_slice(arr);
          Ok(())
        },
        PixelFormatEnum::ABGR8888 => {
          // 4 values per pixel: alpha, blue, green, red
          for i in (0..pitch * height).step_by(4) {
            data.push(arr[i + 3]); // red
            data.push(arr[i + 2]); // green
            data.push(arr[i + 1]); // blue
            data.push(arr[i]); // alpha
          }
          Ok(())
        },
        PixelFormatEnum::RGB24 => {
          // 3 values per pixel
          for i in (0..pitch * height).step_by(3) {
            data.push(arr[i]);
            data.push(arr[i + 1]);
            data.push(arr[i + 2]);
            data.push(0);
          }
          Ok(())
        },
        pixel_format =>
          Err(format!("{:?} is unsupported", pixel_format)),
      }
    })?;

    let mode = Mode::NORMAL;

    Ok(Self { width, height, mode, data })
  }

  /// Returns sprite width.
  #[inline]
  pub fn width(&self) -> usize {
    self.width
  }

  /// Returns sprite height.
  #[inline]
  pub fn height(&self) -> usize {
    self.height
  }

  /// Returns sprite pitch (for data).
  #[inline]
  pub fn pitch(&self) -> usize {
    self.width * 4
  }

  /// Returns sample mode.
  #[inline]
  pub fn mode(&self) -> Mode {
    self.mode
  }

  /// Sets new sample mode.
  #[inline]
  pub fn set_sample_mode(&mut self, mode: Mode) {
    self.mode = mode;
  }

  /// Returns pixel at position (x, y).
  #[inline]
  pub fn get_pixel(&self, x: usize, y: usize) -> Result<Pixel> {
    if x >= self.width || y >= self.height {
      return Err(PGError::from(format!("Out of bound: ({}, {})", x, y)));
    }

    match self.mode {
      Mode::NORMAL => {
        let i = (y * self.width + x) * 4;
        Ok(Pixel::rgba(self.data[i], self.data[i + 1], self.data[i + 2], self.data[i + 3]))
      },
      Mode::PERIODIC => {
        let i = ((y % self.height) * self.width + (x % self.width)) * 4;
        Ok(Pixel::rgba(self.data[i], self.data[i + 1], self.data[i + 2], self.data[i + 3]))
      }
    }
  }

  /// Sets pixel at position (x, y).
  #[inline]
  pub fn set_pixel(&mut self, x: usize, y: usize, p: Pixel) -> Result<()> {
    if x >= self.width || y >= self.height {
      return Err(PGError::from(format!("Out of bound: ({}, {})", x, y)));
    }

    let i = (y * self.width + x) * 4;
    self.data[i] = p.r();
    self.data[i + 1] = p.g();
    self.data[i + 2] = p.b();
    self.data[i + 3] = p.a();

    Ok(())
  }

  #[inline]
  pub fn sample(&self, x: f32, y: f32) -> Result<Pixel> {
    let sx = std::cmp::min((x * self.width as f32) as usize, self.width - 1);
    let sy = std::cmp::min((y * self.height as f32) as usize, self.height - 1);
    // sx and sy are guaranteed to be within bounds
    self.get_pixel(sx, sy)
  }
}
