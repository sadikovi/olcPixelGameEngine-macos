// extern crate sdl2;
// extern crate rand;
// extern crate olc_port;
//
// use std::time::Duration;
// use std::time::Instant;
//
// use sdl2::pixels::Color;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::render::TextureCreator;
// use sdl2::rect::Rect;
// use sdl2::rect::Point;
// use sdl2::mouse::Cursor;
// use sdl2::surface::Surface;
// use sdl2::image::LoadSurface;
// use sdl2::video::Window;
// use sdl2::render::Canvas;
// use sdl2::pixels::PixelFormatEnum::RGBA8888;
//
// use olc_port::sprite::Sprite;
//
// trait Game {
//   fn title(&self) -> &str;
//   fn on_user_create(&mut self);
//   fn on_user_update(&mut self, canvas: &mut Canvas<Window>, width: u32, height: u32, lapsed_time: f32);
//   fn on_user_destroy(&mut self);
// }
//
// struct Example {
// }
//
// impl Game for Example {
//   fn title(&self) -> &str {
//     "Example Game"
//   }
//
//   fn on_user_create(&mut self) {
//     println!("on_user_create");
//   }
//
//   fn on_user_update(&mut self, canvas: &mut Canvas<Window>, width: u32, height: u32, lapsed_time: f32) {
//     println!("on_user_update");
//
//     for x in 0..width {
//       for y in 0..height {
//         let r = rand::random::<u8>();
//         let g = rand::random::<u8>();
//         let b = rand::random::<u8>();
//         let c = Color::RGB(r, g, b);
//         canvas.set_draw_color(c);
//         canvas.draw_point(Point::new(x as i32, y as i32));
//       }
//     }
//   }
//
//   fn on_user_destroy(&mut self) {
//     println!("on_user_destroy");
//   }
// }
//
// struct GameEngine {
//   screen_width: u32,
//   screen_height: u32,
//   pixel_width: u32,
//   pixel_height: u32,
//   full_screen: bool,
// }
//
// impl GameEngine {
//   pub fn screen_width(&self) -> u32 {
//     self.screen_width
//   }
//
//   pub fn screen_height(&self) -> u32 {
//     self.screen_height
//   }
// }
//
// struct EventLoop {
// }
//
// impl EventLoop {
//   pub fn start<'a>(
//     screen_width: u32,
//     screen_height: u32,
//     pixel_width: u32,
//     pixel_height: u32,
//     full_screen: bool,
//     game: &'a mut Game
//   ) -> Result<(), String> {
//     // TODO: check boundaries for width and height
//     let engine = GameEngine { screen_width, screen_height, pixel_width, pixel_height, full_screen };
//
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;
//
//     let mut window_builder = video_subsystem.window(game.title(), screen_width * pixel_width, screen_height * pixel_height);
//
//     window_builder
//       .position_centered()
//       .opengl();
//
//     if full_screen {
//       window_builder.fullscreen();
//     }
//
//     let mut window = window_builder.build().map_err(|e| e.to_string())?;
//     let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
//
//     let texture_creator: TextureCreator<_> = canvas.texture_creator();
//
//     println!("logical size: {:?}", canvas.logical_size());
//
//     // canvas.set_draw_color(Color::RGB(255, 255, 255));
//     canvas.set_scale(pixel_width as f32, pixel_height as f32);
//
//     let image = Surface::from_file(
//       "purple_square.png"
//       // "isometric_demo.png"
//       // "FloatingEye6Eyed.png"
//     ).unwrap();
//     println!("rect: {:?}", image.rect());
//     println!("pixel format enum: {:?}", image.pixel_format_enum());
//     println!("pitch: {}", image.pitch());
//     image.with_lock(|x: &[u8]| {
//       println!("pixels.len: {}", x.len());
//       println!("pixels: {:?}", x);
//     });
//
//     let sprite = Sprite::from_image("purple_square.png").unwrap();
//     println!("width: {}", sprite.width());
//     println!("height: {}", sprite.height());
//     println!("pitch: {}", sprite.pitch());
//     println!("mode: {:?}", sprite.mode());
//
//     for i in 0..sprite.width() {
//       for j in 0..sprite.height() {
//         println!("p[{}, {}]: {:?}", i, j, sprite.get_pixel(i, j));
//       }
//     }
//
//     // let mut rect = image.rect();
//     // rect.reposition((20, 20));
//     // // rect.resize(rect.width() * pixel_width, rect.height() * pixel_height);
//     //
//     // let mut texture = texture_creator.create_texture_from_surface(image).unwrap();
//     //
//     // canvas.copy(&texture, None, Some(rect));
//
//     // canvas.with_texture_canvas(&mut texture, |texture_canvas| {
//     //
//     // }).unwrap();
//
//     // canvas.clear();
//     canvas.present();
//
//     // let mut event_pump = sdl_context.event_pump()?;
//     //
//     // game.on_user_create();
//     //
//     // let mut timer = Instant::now();
//     // let mut frame_count: u32 = 0;
//     // let mut frame_timer: f32 = 0.0;
//     //
//     // 'eventloop: loop {
//     //   for event in event_pump.poll_iter() {
//     //     match event {
//     //       Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//     //         break 'eventloop;
//     //       },
//     //       _ => {
//     //         println!("Event: {:?}", event);
//     //       }
//     //     }
//     //   }
//     //
//     //   let new_timer = Instant::now();
//     //   let elapsed_time = new_timer.duration_since(timer).as_secs_f32();
//     //   timer = new_timer;
//     //
//     //   draw_string(0, 0, "text", Color::RGB(255, 255, 0), 1);
//     //
//     //   canvas.clear();
//     //   game.on_user_update(&mut canvas, screen_width, screen_height, elapsed_time);
//     //   canvas.present();
//     //
//     //   ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
//     //   // The rest of the game loop goes here...
//     //
//     //   println!("elapsed time: {}, frame_count: {}", elapsed_time, frame_count);
//     //
//     //   frame_timer += elapsed_time;
//     //   frame_count += 1;
//     //
//     //   if (frame_timer >= 1.0) {
//     //     canvas.window_mut().set_title(&format!("{} - FPS: {}", game.title(), frame_count));
//     //     frame_timer = 0.0;
//     //     frame_count = 0;
//     //   }
//     // }
//     //
//     // game.on_user_destroy();
//
//     Ok(())
//   }
// }
//
// pub fn main() {
//   let mut example = Example { };
//   EventLoop::start(80, 40, 10, 10, false, &mut example).unwrap();
// }

pub fn main() {

}
