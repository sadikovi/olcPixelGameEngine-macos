use libc::c_float;
use libc::c_char;
use libc::c_void;
use std::ffi::CString;
use std::fmt;

// C API

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum RCode {
  CONSTRUCT_FAIL,
  CONSTRUCT_NO_FILE,
  START_FAIL,
  START_NO_FILE,
  OK
}

#[link(name="olcRustBindingApp", kind="static")]
#[allow(dead_code)]
extern "C" {
  fn create() -> *const c_void;
  fn start(name: *const c_char, binding: *mut c_void, screen_w: i32, screen_h: i32, pixel_w: i32, pixel_h: i32, full_screen: bool, vsync: bool) -> RCode;
}

#[no_mangle]
extern "C" fn onUserCreate(app: *const c_void, binding: *mut c_void) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let ctx = Context { app };
  let res = match b.game.on_user_create(&ctx) {
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
extern "C" fn onUserUpdate(app: *const c_void, binding: *mut c_void, elapsed_time: c_float) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let ctx = Context { app };
  let res = match b.game.on_user_update(&ctx, elapsed_time) {
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
extern "C" fn onUserDestroy(_app: *const c_void, binding: *mut c_void) -> bool {
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
  fn on_user_create(&mut self, ctx: &Context) -> Result<(), OlcError>;
  /// Called on user update action for every frame.
  fn on_user_update(&mut self, ctx: &Context, elapsed_time: f32) -> Result<(), OlcError>;
  /// Called on user destroy action.
  fn on_user_destroy(&mut self) -> Result<(), OlcError>;
}

/// Binding for the game.
struct Binding<'a> {
  game: &'a mut dyn Game
}

/// Context to access drawing routines in olcPixelGameEngine.
pub struct Context {
  // Pointer to the app instance
  app: *const c_void
}

/// Main function to run the game.
/// It is recommended to pass full_screen and vsync as "false".
pub fn run<'a>(
  game: &'a mut dyn Game,
  screen_width: u32,
  screen_height: u32,
  pixel_width: u32,
  pixel_height: u32,
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
      screen_width as i32,
      screen_height as i32,
      pixel_width as i32,
      pixel_height as i32,
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
  fn on_user_create(&mut self, _ctx: &Context) -> Result<(), OlcError> {
    println!("Create call from the game!");
    Ok(())
  }
  fn on_user_update(&mut self, _ctx: &Context, elapsed_time: f32) -> Result<(), OlcError> {
    println!("Update call from the game! {}", elapsed_time);
    Ok(())
  }
  fn on_user_destroy(&mut self) -> Result<(), OlcError> {
    println!("Destroy call from the game!");
    Ok(())
  }
}

fn main() {
  let mut game = Example {};
  run(&mut game, 200, 80, 4, 4, false, false).unwrap();
}
