extern crate sdl2;
extern crate rand;

use std::time::Duration;
use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::rect::Rect;
use sdl2::mouse::Cursor;
use sdl2::surface::Surface;
use sdl2::image::LoadSurface;

const title: &str = "Demo";

#[derive(Clone, Copy, Debug)]
struct Pixel {
  r: u32,
  g: u32,
  b: u32,
  a: u32,
}

impl Pixel {
  pub fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
    Self { r, g, b, a }
  }
}

trait Game {
  fn title(&self) -> &str;
  fn on_user_create(&mut self);
  fn on_user_update(&mut self, engine: &GameEngine, lapsed_time: f32);
  fn on_user_destroy(&mut self);
}

struct Example {
}

impl Game for Example {
  fn title(&self) -> &str {
    "Example Game"
  }

  fn on_user_create(&mut self) {
    println!("on_user_create");
  }

  fn on_user_update(&mut self, engine: &GameEngine, lapsed_time: f32) {
    println!("on_user_update");
  }

  fn on_user_destroy(&mut self) {
    println!("on_user_destroy");
  }
}

struct GameEngine {
  screen_width: u32,
  screen_height: u32,
  pixel_width: u32,
  pixel_height: u32,
  full_screen: bool,
}

impl GameEngine {
  pub fn screen_width(&self) -> u32 {
    self.screen_width
  }

  pub fn screen_height(&self) -> u32 {
    self.screen_height
  }

  pub fn draw(&self, x: u32, y: u32, pixel: Pixel) {
    // TODO
  }
}

struct EventLoop {
}

impl EventLoop {
  pub fn start<'a>(
    screen_width: u32,
    screen_height: u32,
    pixel_width: u32,
    pixel_height: u32,
    full_screen: bool,
    game: &'a mut Game
  ) -> Result<(), String> {
    // TODO: check boundaries for width and height
    let engine = GameEngine { screen_width, screen_height, pixel_width, pixel_height, full_screen };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window_builder = video_subsystem.window(game.title(), screen_width * pixel_width, screen_height * pixel_height);

    window_builder
      .position_centered()
      .opengl();

    if full_screen {
      window_builder.fullscreen();
    }

    let mut window = window_builder.build().map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // let surface = Surface::from_file("/Users/sadikovi/Downloads/FloatingEye6Eyed.png")
    //   .map_err(|err| format!("failed to load cursor image: {}", err))?;
    // let cursor = Cursor::from_surface(surface, 0, 0)
    //   .map_err(|err| format!("failed to load cursor: {}", err))?;
    // cursor.set();

    let mut texture = texture_creator.create_texture_target(None, 54, 54).unwrap();

    // canvas.with_texture_canvas(&mut texture, |texture_canvas| {
    //   texture_canvas.set_draw_color(Color::RGB(155, 155, 155));
    //   texture_canvas.clear();
    //   texture_canvas.set_draw_color(Color::RGB(255, 0, 0));
    //   texture_canvas.fill_rect(Rect::new(0, 0, 10, 10)).unwrap();
    // }).unwrap();

    println!("logical size: {:?}", canvas.logical_size());

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.clear();
    // canvas.copy(&texture, None, Some(Rect::new(10, 10, 100, 100))).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    game.on_user_create();

    let mut timer = Instant::now();
    let mut frame_count: u32 = 0;
    let mut frame_timer: f32 = 0.0;

    'eventloop: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            break 'eventloop;
          },
          _ => {
            println!("Event: {:?}", event);
          }
        }
      }

      let new_timer = Instant::now();
      let elapsed_time = new_timer.duration_since(timer).as_secs_f32();
      timer = new_timer;

      game.on_user_update(&engine, elapsed_time);

      // canvas.clear();

      for x in 0..engine.screen_width {
        for y in 0..engine.screen_height {
          let r = rand::random::<u8>();
          let g = rand::random::<u8>();
          let b = rand::random::<u8>();

          println!("colour: {:?}", Color::RGB(r, g, b));

          canvas.set_draw_color(Color::RGB(r, g, b));
          canvas.fill_rect(Rect::new((x * engine.pixel_width) as i32, (y * engine.pixel_height) as i32, engine.pixel_width, engine.pixel_height));
        }
      }

      canvas.present();

      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
      // The rest of the game loop goes here...

      println!("elapsed time: {}, frame_count: {}", elapsed_time, frame_count);

      frame_timer += elapsed_time;
      frame_count += 1;

      if (frame_timer >= 1.0) {
        canvas.window_mut().set_title(&format!("{} - FPS: {}", game.title(), frame_count));
        frame_timer = 0.0;
        frame_count = 0;
      }
    }

    game.on_user_destroy();

    Ok(())
  }
}

pub fn main() -> Result<(), String> {
  let mut example = Example { };
  EventLoop::start(200, 100, 4, 4, false, &mut example)
}
