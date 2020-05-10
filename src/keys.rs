use sdl2::event::Event;
use sdl2::mouse::MouseButton;

/// Different mouse buttons that are supported.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseBtn {
  LEFT,
  MIDDLE,
  RIGHT,
  X1,
  X2,
}

/// Generic state of a key or a mouse button.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct State {
  pub is_pressed: bool,
  pub is_released: bool
}

/// Mouse state, contains information on all of the mouse buttons.
#[derive(Clone, Copy, Debug)]
pub struct MouseState {
  mouse_x: usize,
  mouse_y: usize,
  states: [State; 5]
}

impl MouseState {
  /// Initialises new mouse state.
  pub fn new() -> Self {
    Self { mouse_x: 0, mouse_y: 0, states: [State::default(); 5] }
  }

  /// Returns x mouse coordinate.
  #[inline]
  pub fn mouse_x(&self) -> usize {
    self.mouse_x
  }

  /// Returns y mouse coordinate.
  #[inline]
  pub fn mouse_y(&self) -> usize {
    self.mouse_y
  }

  /// Returns button state.
  #[inline]
  pub fn button(&self, btn: MouseBtn) -> &State {
    let id = Self::mouse_btn_to_id(btn);
    &self.states[id]
  }

  /// Resets the state.
  #[inline]
  pub fn reset(&mut self) {
    for i in 0..self.states.len() {
      self.states[i].is_pressed = false;
      self.states[i].is_released = false;
    }
  }

  /// Updates the current state.
  /// This is an internal method.
  pub fn update(&mut self, event: Event) {
    match event {
      Event::MouseMotion { x, y, mousestate, .. } => {
        self.mouse_x = x as usize;
        self.mouse_y = y as usize;

        self.reset();

        if mousestate.left() {
          self.states[Self::mouse_btn_to_id(MouseBtn::LEFT)].is_pressed = true;
        }
        if mousestate.middle() {
          self.states[Self::mouse_btn_to_id(MouseBtn::MIDDLE)].is_pressed = true;
        }
        if mousestate.right() {
          self.states[Self::mouse_btn_to_id(MouseBtn::RIGHT)].is_pressed = true;
        }
        if mousestate.x1() {
          self.states[Self::mouse_btn_to_id(MouseBtn::X1)].is_pressed = true;
        }
        if mousestate.x2() {
          self.states[Self::mouse_btn_to_id(MouseBtn::X2)].is_pressed = true;
        }
      },
      Event::MouseButtonDown { mouse_btn, .. } => {
        if let Some(btn) = Self::mouse_button_to_btn(mouse_btn) {
          let id = Self::mouse_btn_to_id(btn);
          self.states[id].is_released = false;
          self.states[id].is_pressed = true;
        }
      },
      Event::MouseButtonUp { mouse_btn, .. } => {
        if let Some(btn) = Self::mouse_button_to_btn(mouse_btn) {
          let id = Self::mouse_btn_to_id(btn);
          self.states[id].is_pressed = false;
          self.states[id].is_released = true;
        }
      },
      _ => {
        // Do nothing for now.
      }
    }
  }

  /// Internal method to convert SDL2 mouse button into mouse button.
  #[inline]
  fn mouse_button_to_btn(btn: MouseButton) -> Option<MouseBtn> {
    match btn {
      MouseButton::Left => Some(MouseBtn::LEFT),
      MouseButton::Middle => Some(MouseBtn::MIDDLE),
      MouseButton::Right => Some(MouseBtn::RIGHT),
      MouseButton::X1 => Some(MouseBtn::X1),
      MouseButton::X2 => Some(MouseBtn::X2),
      _ => None
    }
  }

  /// Internal method to convert mouse button to an array index.
  #[inline]
  fn mouse_btn_to_id(btn: MouseBtn) -> usize {
    match btn {
      MouseBtn::LEFT => 0,
      MouseBtn::MIDDLE => 1,
      MouseBtn::RIGHT => 2,
      MouseBtn::X1 => 3,
      MouseBtn::X2 => 4
    }
  }
}
