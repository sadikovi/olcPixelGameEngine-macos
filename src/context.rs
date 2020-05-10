use crate::error::Result;
use crate::pixel::{Pixel, PixelMode};
use crate::sprite::Sprite;

/// Context provides facilities to draw objects.
pub struct Context {
  screen_width: usize, // screen width in "pixels"
  screen_height: usize, // screen height in "pixels"
  current: Sprite, // current drawing target
  font: Sprite, // sprite for font
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
      font: load_font(),
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

  /// Draws string in the current drawing target.
  pub fn draw_string(&mut self, x: usize, y: usize, text: &str, col: Pixel, scale: usize) -> Result<()> {
    let mut sx = 0;
    let mut sy = 0;

    let m = self.pixel_mode;

    if col.a() != 255 {
      self.set_pixel_mode(PixelMode::ALPHA);
    } else {
      self.set_pixel_mode(PixelMode::MASK);
    }

    for c in text.chars() {
      if c == '\n' {
        sx = 0;
        sy += 8 * scale;
      } else {
        let ox = (c as usize - 32) % 16;
        let oy = (c as usize - 32) / 16;

        if scale > 1 {
          for i in 0..8 {
            for j in 0..8 {
              if self.font.get_pixel(i + ox * 8, j + oy * 8)?.r() > 0 {
                for is in 0..scale {
                  for js in 0..scale {
                    self.draw(x + sx + (i * scale) + is, y + sy + (j * scale) + js, col)?;
                  }
                }
              }
            }
          }
        } else {
          for i in 0..8 {
            for j in 0..8 {
              if self.font.get_pixel(i + ox * 8, j + oy * 8)?.r() > 0 {
                self.draw(x + sx + i, y + sy + j, col)?;
              }
            }
          }
        }

        sx += 8 * scale;
      }
    }

    self.set_pixel_mode(m);

    Ok(())
  }
}

const FONT: &str =
  "?Q`0001oOch0o01o@F40o0<AGD4090LAGD<090@A7ch0?00O7Q`0600>00000000\
  O000000nOT0063Qo4d8>?7a14Gno94AA4gno94AaOT0>o3`oO400o7QN00000400\
  Of80001oOg<7O7moBGT7O7lABET024@aBEd714AiOdl717a_=TH013Q>00000000\
  720D000V?V5oB3Q_HdUoE7a9@DdDE4A9@DmoE4A;Hg]oM4Aj8S4D84@`00000000\
  OaPT1000Oa`^13P1@AI[?g`1@A=[OdAoHgljA4Ao?WlBA7l1710007l100000000\
  ObM6000oOfMV?3QoBDD`O7a0BDDH@5A0BDD<@5A0BGeVO5ao@CQR?5Po00000000\
  Oc``000?Ogij70PO2D]??0Ph2DUM@7i`2DTg@7lh2GUj?0TO0C1870T?00000000\
  70<4001o?P<7?1QoHg43O;`h@GT0@:@LB@d0>:@hN@L0@?aoN@<0O7ao0000?000\
  OcH0001SOglLA7mg24TnK7ln24US>0PL24U140PnOgl0>7QgOcH0K71S0000A000\
  00H00000@Dm1S007@DUSg00?OdTnH7YhOfTL<7Yh@Cl0700?@Ah0300700000000\
  <008001QL00ZA41a@6HnI<1i@FHLM81M@@0LG81?O`0nC?Y7?`0ZA7Y300080000\
  O`082000Oh0827mo6>Hn?Wmo?6HnMb11MP08@C11H`08@FP0@@0004@000000000\
  00P00001Oab00003OcKP0006@6=PMgl<@440MglH@000000`@000001P00000000\
  Ob@8@@00Ob@8@Ga13R@8Mga172@8?PAo3R@827QoOb@820@0O`0007`0000007P0\
  O`000P08Od400g`<3V=P0G`673IP0`@3>1`00P@6O`P00g`<O`000GP800000000\
  ?P9PL020O`<`N3R0@E4HC7b0@ET<ATB0@@l6C4B0O`H3N7b0?P01L3R000000020";

/// Loads font into a sprite.
fn load_font() -> Sprite {
  let mut sprite = Sprite::new(128, 48);

  let mut px = 0;
  let mut py = 0;
  let data = FONT.as_bytes();

  for b in (0..1024).step_by(4) {
    let sym1: usize = data[b + 0] as usize - 48;
    let sym2: usize = data[b + 1] as usize - 48;
    let sym3: usize = data[b + 2] as usize - 48;
    let sym4: usize = data[b + 3] as usize - 48;
    let r = sym1 << 18 | sym2 << 12 | sym3 << 6 | sym4;

    for i in 0..24 {
      let k = if (r & (1 << i)) > 0 { 255 } else { 0 };
      sprite.set_pixel(px, py, Pixel::rgba(k, k, k, k)).unwrap();
      py += 1;
      if py == 48 {
        px += 1;
        py = 0;
      }
    }
  }

  sprite
}
