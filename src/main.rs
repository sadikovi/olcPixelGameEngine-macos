use libc::c_float;
use libc::c_char;
use libc::c_void;
use std::ffi::CString;
use std::fmt;

// C API

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum RCode {
  FAIL = 0,
  OK = 1,
  NO_FILE = -1
}

#[no_mangle]
extern "C" fn onUserCreate(ptr: *const c_void) -> bool {
  println!("onUserCreate: {:?}", ptr);
  return true;
}

#[no_mangle]
extern "C" fn onUserUpdate(ptr: *const c_void, fElapsedTime: c_float) -> bool {
  println!("onUserUpdate: {:?}, {}", ptr, fElapsedTime);
  return true;
}

#[no_mangle]
extern "C" fn onUserDestroy(ptr: *const c_void) -> bool {
  println!("onUserDestroy: {:?}", ptr);
  return true;
}

#[link(name="olcRustBinding", kind="static")]
extern "C" {
  fn create(name: *const c_char) -> *mut c_void;
  fn construct(ptr: *mut c_void, screen_w: i32, screen_h: i32, pixel_w: i32, pixel_h: i32, full_screen: bool, vsync: bool) -> RCode;
  fn start(ptr: *mut c_void) -> RCode;
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

pub trait Game {
  fn name(&self) -> &str;
  fn on_user_create(&mut self) -> Result<(), OlcError>;
  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), OlcError>;
  fn on_user_destroy(&mut self) -> Result<(), OlcError>;
}

pub struct GameEngine<'a> {
  // Pointer to the engine instance
  ptr: *mut c_void,
  game: &'a mut dyn Game,
}

impl<'a> GameEngine<'a> {
  pub fn create(game: &'a mut dyn Game) -> Result<Self, OlcError> {
    let name = CString::new(game.name())?;
    let ptr = unsafe { create(name.as_ptr()) };
    Ok(Self { ptr: ptr, game: game })
  }

  pub fn start(
    self,
    screen_width: u32,
    screen_height: u32,
    pixel_width: u32,
    pixel_height: u32
  ) -> Result<(), OlcError>
  {
    self.start_advanced(screen_width, screen_height, pixel_width, pixel_height, false, false)
  }

  pub fn start_advanced(
    self,
    screen_width: u32,
    screen_height: u32,
    pixel_width: u32,
    pixel_height: u32,
    full_screen: bool,
    vsync: bool
  ) -> Result<(), OlcError>
  {
    let res = unsafe {
      construct(
        self.ptr,
        screen_width as i32,
        screen_height as i32,
        pixel_width as i32,
        pixel_height as i32,
        full_screen,
        vsync
      )
    };

    if res == RCode::FAIL {
      return Err(OlcError { msg: format!("Failed to construct the application: FAIL") });
    } else if res == RCode::NO_FILE {
      return Err(OlcError { msg: format!("Failed to construct the application: NO_FILE") });
    }

    let res = unsafe { start(self.ptr) };

    if res == RCode::FAIL {
      return Err(OlcError { msg: format!("Failed to start the application: FAIL") });
    } else if res == RCode::NO_FILE {
      return Err(OlcError { msg: format!("Failed to start the application: NO_FILE") });
    }

    Ok(())
  }
}

struct Example {}
impl Game for Example {
  fn name(&self) -> &str { "Hello, World!" }
  fn on_user_create(&mut self) -> Result<(), OlcError> { Ok(()) }
  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), OlcError> { Ok(()) }
  fn on_user_destroy(&mut self) -> Result<(), OlcError> { Ok(()) }
}

fn main() {
  let mut game = Example {};
  let engine = GameEngine::create(&mut game).unwrap();
  engine.start(200, 80, 4, 4).unwrap();
}
