use libc::c_float;
use libc::c_char;
use libc::c_void;
use std::ffi::CString;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum rcode {
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

#[link(name="olcBinding", kind="static")]
extern "C" {
  fn create(name: *const c_char) -> *mut c_void;
  fn construct(ptr: *mut c_void, screen_w: i32, screen_h: i32, pixel_w: i32, pixel_h: i32, full_screen: bool, vsync: bool) -> rcode;
  fn start(ptr: *mut c_void) -> rcode;
}

fn main() {
  // let pixel = unsafe { PixelF(1.0, 0.5, 0.4, 0.3) };
  // println!("pixel: {:?}", pixel);
  let name = CString::new("Hello, World!").unwrap();
  let ptr = unsafe { create(name.as_ptr()) };
  println!("created ptr: {:?}", ptr);
  let res = unsafe { construct(ptr, 200, 80, 4, 4, false, false) };
  println!("construct result: {:?}", res);
  if res == rcode::OK {
    let res = unsafe { start(ptr) };
    println!("start result: {:?}", res);
  }
}
