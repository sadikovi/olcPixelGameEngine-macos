use libc::c_float;
use libc::c_char;
use libc::c_void;
use std::ffi::CString;
use std::fmt;

// C API

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum RCode {
  CONSTRUCT_FAIL,
  CONSTRUCT_NO_FILE,
  START_FAIL,
  START_NO_FILE,
  OK
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8
}

#[link(name="olcRustBindingApp", kind="static")]
#[allow(dead_code)]
extern "C" {
  fn c_rand() -> i32;
  fn start(name: *const c_char, binding: *mut c_void, screen_w: i32, screen_h: i32, pixel_w: i32, pixel_h: i32, full_screen: bool, vsync: bool) -> RCode;
  fn draw(x: i32, y: i32, p: Pixel) -> bool;
  fn screen_width() -> i32;
  fn screen_height() -> i32;
}

#[no_mangle]
extern "C" fn onUserCreate(binding: *mut c_void) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let res = match b.game.on_user_create() {
    Err(err) => {
      println!("ERROR: {}", err);
      false
    },
    Ok(_) => true
  };
  Box::leak(b); // always leak the binding, it will be cleaned up in the main function
  res
}

#[no_mangle]
extern "C" fn onUserUpdate(binding: *mut c_void, elapsed_time: c_float) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let res = match b.game.on_user_update(elapsed_time) {
    Err(err) => {
      println!("ERROR: {}", err);
      false
    },
    Ok(_) => true
  };
  Box::leak(b); // always leak the binding, it will be cleaned up in the main function
  res
}

#[no_mangle]
extern "C" fn onUserDestroy(binding: *mut c_void) -> bool {
  // binding goes out of scope and is dropped
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  match b.game.on_user_destroy() {
    Err(err) => println!("ERROR: {}", err),
    Ok(_) => {}
  }
  true // always return true to finish cleanup
}

// Rust API

#[derive(Clone, Debug)]
pub struct OlcError {
  msg: String
}

impl fmt::Display for OlcError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl From<std::ffi::NulError> for OlcError {
  fn from(error: std::ffi::NulError) -> Self {
    Self { msg: format!("{}", error) }
  }
}

/// Game trait, should be extended by an implementation and passed to run function.
pub trait Game {
  /// Returns the name of the application.
  fn name(&self) -> &str;
  /// Called on user create action.
  fn on_user_create(&mut self) -> Result<(), OlcError>;
  /// Called on user update action for every frame.
  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), OlcError>;
  /// Called on user destroy action.
  fn on_user_destroy(&mut self) -> Result<(), OlcError>;
}

/// Binding for the game.
struct Binding<'a> {
  game: &'a mut dyn Game
}

/// Main function to run the game.
/// It is recommended to pass full_screen and vsync as "false".
pub fn run<'a>(
  game: &'a mut dyn Game,
  screen_width: i32,
  screen_height: i32,
  pixel_width: i32,
  pixel_height: i32,
  full_screen: bool,
  vsync: bool
) -> Result<(), OlcError>
{
  let name = CString::new(game.name())?;

  let binding = Binding { game };

  let res = unsafe {
    start(
      name.as_ptr(),
      Box::into_raw(Box::new(binding)) as *mut c_void,
      screen_width,
      screen_height,
      pixel_width,
      pixel_height,
      full_screen,
      vsync
    )
  };

  match res {
    RCode::CONSTRUCT_FAIL =>
      Err(OlcError { msg: format!("Failed to construct the application: FAIL") }),
    RCode::CONSTRUCT_NO_FILE =>
      Err(OlcError { msg: format!("Failed to construct the application: NO_FILE") }),
    RCode::START_FAIL =>
      Err(OlcError { msg: format!("Failed to start the application: FAIL") }),
    RCode::START_NO_FILE =>
      Err(OlcError { msg: format!("Failed to start the application: NO_FILE") }),
    RCode::OK =>
      Ok(())
  }
}

struct Example {}
impl Game for Example {
  fn name(&self) -> &str { "Hello, World!" }

  fn on_user_create(&mut self) -> Result<(), OlcError> {
    println!("Create call from the game!");
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), OlcError> {
    // println!("Update call from the game! {}", elapsed_time);
    unsafe {
      for x in 0..screen_width() {
        for y in 0..screen_height() {
          let p = Pixel {
            r: (c_rand() % 255) as u8,
            g: (c_rand() % 255) as u8,
            b: (c_rand() % 255) as u8,
            a: 255
          };
          draw(x, y, p);
        }
      }
    }

    Ok(())
  }

  #[inline]
  fn on_user_destroy(&mut self) -> Result<(), OlcError> {
    println!("Destroy call from the game!");
    Ok(())
  }
}

fn main() {
  let mut game = Example {};
  run(&mut game, 256, 240, 4, 4, false, false).unwrap();
}
