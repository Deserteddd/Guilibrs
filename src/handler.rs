use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::{EventPump, Sdl};

pub struct EventHandler {
    pump: EventPump,
    hovered: Option<usize>,
    lmb_down: (bool, i32, i32), // (is_down, x, y)
}

impl EventHandler {
    pub fn new(context: &Sdl) -> Result<EventHandler, String> {
        Ok(EventHandler {
            pump: context.event_pump()?,
            hovered: None,
            lmb_down: (false, 0, 0),
        })
    }

    pub fn poll(&mut self, bounds: &[Rect]) -> HandlerEvent {
        match self.pump.wait_event() {
            Event::Quit { .. } => HandlerEvent::Quit,
            Event::TextInput { text, .. } => HandlerEvent::TextInput(text),

            Event::KeyDown { keycode, .. } => {
                self.parse_keycode(keycode)
            },
            Event::MouseMotion { x, y, .. } => {
                if let Some(idx) = self.hovered {
                    if self.lmb_down.0 {
                        return HandlerEvent::Drag(idx, x, y)
                    }
                    if !in_bounds(&bounds[idx], x, y) {
                        self.hovered = None;
                        return HandlerEvent::UnHover(idx)
                    }
                } else {
                    for (idx, b) in bounds.iter().enumerate() {
                        if in_bounds(b, x, y) {
                            self.hovered = Some(idx);
                            return HandlerEvent::Hover(idx);
                        }
                    }
                }
                HandlerEvent::None
            },
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if mouse_btn == MouseButton::Left {
                    self.lmb_down = (true, x, y);
                }
                HandlerEvent::None
            },
            Event::MouseButtonUp { mouse_btn, .. } => {
                if mouse_btn == MouseButton::Left {
                    self.lmb_down.0 = false;
                    if let Some(idx) = self.hovered {
                        return HandlerEvent::Click(idx)
                    } else {
                        return HandlerEvent::ClickBackround
                    }
                }
                HandlerEvent::None
            }
            _ => HandlerEvent::None,
        }
    }

    const fn parse_keycode(&self, kc: Option<Keycode>) -> HandlerEvent {
        if let Some(keycode) = kc {
            return match keycode {
                Keycode::Backspace => HandlerEvent::PopChar,
                Keycode::Tab => HandlerEvent::TabPress,
                Keycode::Return => HandlerEvent::Return,
                Keycode::Escape => HandlerEvent::Escape,
                _ => HandlerEvent::None,
            }
        }
        HandlerEvent::None
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandlerEvent {
    Quit,
    Hover(usize),
    UnHover(usize),
    Click(usize),
    Drag(usize, i32, i32),
    Escape,
    Return,
    TextInput(String),
    PopChar,
    ClickBackround,
    TabPress,
    None
}

fn in_bounds(rect: &Rect, x: i32, y: i32) -> bool {
    if x >= rect.x && x <= rect.x + rect.w && y >= rect.y && y <= rect.y + rect.h {
        return true;
    }
    false
}
