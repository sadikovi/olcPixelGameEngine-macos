pub mod error;
pub mod pixel;
pub mod keys;
pub mod sprite;
pub mod context;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;

use crate::error::{PGError, Result};
use crate::context::Context;

pub trait Game {
  fn title(&self) -> &str;
  fn on_user_create(&mut self) -> Result<()>;
  fn on_user_update(&mut self, context: &mut Context, elapsed_time: f32) -> Result<()>;
  fn on_user_destroy(&mut self) -> Result<()>;
}

pub fn start_event_loop<'a>(
  screen_width: usize,
  screen_height: usize,
  pixel_width: usize,
  pixel_height: usize,
  full_screen: bool,
  game: &'a mut dyn Game
) -> Result<()> {
  if screen_width == 0 || screen_height == 0 {
    return Err(PGError::from(
      format!("Invalid screen size: ({}, {})", screen_width, screen_height)));
  }

  if pixel_width == 0 || pixel_height == 0 ||
      pixel_width > screen_width || pixel_height > screen_height {
    return Err(PGError::from(
      format!("Invalid pixel size: ({}, {})", pixel_width, pixel_height)));
  }

  let sdl_context = sdl2::init()?;
  let video = sdl_context.video()?;

  let window_width = (screen_width * pixel_width) as u32;
  let window_height = (screen_height * pixel_height) as u32;

  // Configure window.
  let mut window_builder = video.window(game.title(), window_width, window_height);

  window_builder
    .position_centered()
    .opengl();

  if full_screen {
    window_builder.fullscreen();
  }

  let mut canvas = window_builder
    .build().map_err(|e| e.to_string())?
    .into_canvas().build().map_err(|e| e.to_string())?;
  canvas.set_scale(pixel_width as f32, pixel_height as f32)?;
  let mut event_pump = sdl_context.event_pump()?;
  let texture_creator: TextureCreator<_> = canvas.texture_creator();
  let mut context = Context::new(screen_width, screen_height);

  canvas.clear();
  canvas.present();

  game.on_user_create()?;

  let mut timer = Instant::now();
  let mut frame_count: u32 = 0;
  let mut frame_timer: f32 = 0.0;

  'eventloop: loop {
    context.mouse_state_mut().reset();

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'eventloop;
        },
        // Mouse coordinates are in "pixel" values, we need to reset them
        Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
          let upd_event = Event::MouseMotion {
            timestamp,
            window_id,
            which,
            mousestate,
            x: x / pixel_width as i32,
            y: y / pixel_height as i32,
            xrel,
            yrel
          };
          context.mouse_state_mut().update(upd_event);
        },
        _ => {
          context.mouse_state_mut().update(event);
          // println!("Event: {:?}", event);
        }
      }
    }

    let new_timer = Instant::now();
    let elapsed_time = new_timer.duration_since(timer).as_secs_f32();
    timer = new_timer;

    game.on_user_update(&mut context, elapsed_time)?;

    let width = context.draw_target().width() as u32;
    let height = context.draw_target().height() as u32;
    let pitch = context.draw_target().pitch() as u32;
    let pixel_format = context.draw_target().pixel_format();

    let frame = Surface::from_data(
      context.draw_target_mut().data_mut(),
      width,
      height,
      pitch,
      pixel_format
    )?;

    let rect = frame.rect();

    let texture = texture_creator.create_texture_from_surface(frame)
      .map_err(|e| e.to_string())?;

    canvas.clear();
    canvas.copy(&texture, None, Some(rect))?;
    canvas.present();

    frame_timer += elapsed_time;
    frame_count += 1;

    if frame_timer >= 1.0 {
      canvas.window_mut().set_title(&format!("{} - FPS: {}", game.title(), frame_count))
        .map_err(|e| e.to_string())?;
      frame_timer = 0.0;
      frame_count = 0;
    }
  }

  game.on_user_destroy()?;

  Ok(())
}
