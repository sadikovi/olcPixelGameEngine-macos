use olc_port::{Game, start_event_loop};
use olc_port::context::Context;
use olc_port::pixel::Pixel;
use olc_port::error::Result;

struct Example {
}

impl Game for Example {
  fn title(&self) -> &str {
    "Hello World"
  }

  fn on_user_create(&mut self) -> Result<()> {
    Ok(())
  }

  fn on_user_update(&mut self, context: &mut Context, elapsed_time: f32) -> Result<()> {
    let sprite = context.draw_target_mut();
    for x in 0..sprite.width() {
      for y in 0..sprite.height() {
        let r = rand::random::<u8>();
        let g = rand::random::<u8>();
        let b = rand::random::<u8>();
        let p = Pixel::rgb(r, g, b);
        sprite.set_pixel(x, y, p)?;
      }
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<()> {
    Ok(())
  }
}

fn main() {
  let mut game = Example { };
  start_event_loop(100, 100, 4, 4, false, &mut game).unwrap();
}
