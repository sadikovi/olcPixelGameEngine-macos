use olc_port::{Game, start_event_loop};
use olc_port::context::Context;
use olc_port::pixel::Pixel;
use olc_port::error::Result;

struct Example {
}

impl Game for Example {
  fn title(&self) -> &str {
    "Strings"
  }

  fn on_user_create(&mut self) -> Result<()> {
    Ok(())
  }

  fn on_user_update(&mut self, context: &mut Context, _elapsed_time: f32) -> Result<()> {
    context.draw_string(10, 10, "Hello world!\nWelcome to the OLC!", Pixel::rgb(255, 0, 255), 1)
  }

  fn on_user_destroy(&mut self) -> Result<()> {
    Ok(())
  }
}

fn main() {
  let mut game = Example { };
  start_event_loop(200, 100, 4, 4, false, &mut game).unwrap();
}
