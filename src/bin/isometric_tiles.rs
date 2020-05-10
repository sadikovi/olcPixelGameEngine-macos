use olc_port::{Game, start_event_loop};
use olc_port::context::Context;
use olc_port::keys::MouseBtn;
use olc_port::pixel::{Pixel, PixelMode};
use olc_port::sprite::{Flip, Sprite};
use olc_port::error::Result;

struct Example {
  sprite: Sprite,
  world_size: (usize, usize),
  tile_size: (usize, usize),
  origin: (usize, usize),
  world: Vec<usize>
}

impl Example {
  pub fn new() -> Self {
    let sprite = Sprite::from_image("isometric_demo.png").unwrap();
    let world_size = (14, 10);
    Self {
      sprite: sprite,
      world_size: world_size,
      tile_size: (40, 20),
      origin: (5, 1),
      world: vec![0; (world_size.0 * world_size.1) as usize]
    }
  }

  fn to_screen(&self, x: i32, y: i32) -> (i32, i32) {
    (
      (self.origin.0 * self.tile_size.0) as i32 + (x - y) * (self.tile_size.0 as i32 / 2),
      (self.origin.1 * self.tile_size.1) as i32 + (x + y) * (self.tile_size.1 as i32 / 2)
    )
  }
}

impl Game for Example {
  fn title(&self) -> &str {
    "Isometric Tiles"
  }

  fn on_user_create(&mut self) -> Result<()> {
    Ok(())
  }

  fn on_user_update(&mut self, context: &mut Context, _elapsed_time: f32) -> Result<()> {
    context.clear(Pixel::WHITE())?;

    let mouse_x = context.mouse_state().mouse_x();
    let mouse_y = context.mouse_state().mouse_y();

    let cell = (mouse_x / self.tile_size.0, mouse_y / self.tile_size.1);
    let offset = (mouse_x % self.tile_size.0, mouse_y % self.tile_size.1);

    context.set_pixel_mode(PixelMode::MASK);

    for y in 0..self.world_size.1 {
      for x in 0..self.world_size.0 {
        let (wx, wy) = self.to_screen(x as i32, y as i32);
        match self.world[y * self.world_size.0 + x] {
          // Invisible tile
          0 => {
            context.draw_rect(
              wx as usize,
              wy as usize,
              self.tile_size.0 as usize,
              self.tile_size.1 as usize,
              Pixel::BLACK()
            )?;
            context.draw_partial_sprite(wx as usize, wy as usize, &self.sprite, 1 * self.tile_size.0, 0, self.tile_size.0, self.tile_size.1, 1, Flip::NONE)?;
          },
          _ => unreachable!(),
        }
      }
    }

    // println!("cell: {:?}", cell);
    // println!("offset: {:?}", offset);
    //
    // let mut selected = (
    //   (cell.1 - self.origin.1) + (cell.0 - self.origin.0),
    //   (cell.1 - self.origin.1) - (cell.0 - self.origin.0)
    // );
    //
    // println!("selected: {:?}", selected);
    //
    // if col == Pixel::RED() { selected.0 = selected.0 - 1; selected.1 += 0; }
    // if col == Pixel::BLUE() { selected.0 += 0; selected.1 = selected.1 - 1; }
    // if col == Pixel::GREEN() { selected.0 += 0; selected.1 += 1; }
    // if col == Pixel::YELLOW() { selected.0 += 1; selected.1 += 0; }
    //
    // if context.mouse_state().button(MouseBtn::LEFT).is_pressed {
    //   let sx = selected.0;
    //   let sy = selected.1;
    //
    //   if sx < self.world_size.0 && sy < self.world_size.1 {
    //     let v = self.world[sy * self.world_size.0 + sx] + 1;
    //     self.world[sy * self.world_size.0 + sx] = v % 6;
    //   }
    // }
    //
    // context.set_pixel_mode(PixelMode::MASK);
    //
    // for y in 0..self.world_size.1 {
    //   for x in 0..self.world_size.0 {
    //     let (wx, wy) = self.to_screen(x, y);
    //
    //     match self.world[y * self.world_size.0 + x] {
    //       // Invisible tile
    //       0 => context.draw_partial_sprite(wx, wy, &self.sprite, 1 * self.tile_size.0, 0, self.tile_size.0, self.tile_size.1, 1, Flip::NONE)?,
    //       // Visible tile
    //       1 => context.draw_partial_sprite(wx, wy, &self.sprite, 2 * self.tile_size.0, 0, self.tile_size.0, self.tile_size.1, 1, Flip::NONE)?,
    //       // Tree
    //       2 => context.draw_partial_sprite(wx, wy - self.tile_size.1, &self.sprite, 0, 1 * self.tile_size.1, self.tile_size.0, self.tile_size.1 * 2, 1, Flip::NONE)?,
    //       // Spooky tree
    //       3 => context.draw_partial_sprite(wx, wy - self.tile_size.1, &self.sprite, 1 * self.tile_size.0, 1 * self.tile_size.1, self.tile_size.0, self.tile_size.1 * 2, 1, Flip::NONE)?,
    //       // Beach
    //       4 => context.draw_partial_sprite(wx, wy - self.tile_size.1, &self.sprite, 2 * self.tile_size.0, 1 * self.tile_size.1, self.tile_size.0, self.tile_size.1 * 2, 1, Flip::NONE)?,
    //       // Water
    //       5 => context.draw_partial_sprite(wx, wy - self.tile_size.1, &self.sprite, 3 * self.tile_size.0, 1 * self.tile_size.1, self.tile_size.0, self.tile_size.1 * 2, 1, Flip::NONE)?,
    //       _ => unreachable!(),
    //     }
    //   }
    // }
    //
    // context.set_pixel_mode(PixelMode::ALPHA);
    //
    // let (swx, swy) = self.to_screen(selected.0, selected.1);
    //
    // context.draw_partial_sprite(swx, swy, &self.sprite, 0 * self.tile_size.0, 0, self.tile_size.0, self.tile_size.1, 1, Flip::NONE)?;
    //

    context.set_pixel_mode(PixelMode::NORMAL);

    context.draw_rect(
      (cell.0 * self.tile_size.0) as usize,
      (cell.1 * self.tile_size.1) as usize,
      self.tile_size.0 as usize,
      self.tile_size.1 as usize,
      Pixel::RED()
    );

    context.draw_string(4, 4, &format!("Mouse {}, {}", mouse_x, mouse_y), Pixel::BLACK(), 1)?;
    context.draw_string(4, 14, &format!("Cell {}, {}", cell.0, cell.1), Pixel::BLACK(), 1)?;
    // context.draw_string(4, 24, &format!("Selected {}, {}", selected.0, selected.1), Pixel::BLACK(), 1)?;

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<()> {
    Ok(())
  }
}

fn main() {
  let mut game = Example::new();
  start_event_loop(512, 300, 2, 2, false, &mut game).unwrap();
}
