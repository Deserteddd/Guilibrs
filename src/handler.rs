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
            Event::TextInput { text, .. } => {
                if let Some(c) = text.chars().next() {
                    return HandlerEvent::PushChar(c as u8);
                }
                HandlerEvent::None
            },
            Event::KeyDown { keycode, .. } => {
                self.parse_keycode(keycode)
            },
            Event::MouseMotion { x, y, .. } => {
                let mut hover_instruction: Option<HandlerEvent> = None;
                if let Some(idx) = self.hovered {
                    if !in_bounds(&bounds[idx], x, y) {
                        hover_instruction = Some(HandlerEvent::UnHover(idx))
                    }
                } else {
                    for (idx, b) in bounds.iter().enumerate() {
                        if in_bounds(b, x, y) {
                            hover_instruction = Some(HandlerEvent::Hover(idx));
                        }
                    }
                }
                match hover_instruction {
                    Some(HandlerEvent::Hover(idx)) => {
                        self.hovered = Some(idx);
                        HandlerEvent::Hover(idx)
                    }
                    Some(HandlerEvent::UnHover(idx)) => {
                        self.hovered = None;
                        HandlerEvent::UnHover(idx)
                    }
                    Some(_) => HandlerEvent::None,
                    None => HandlerEvent::None,
                }
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

    fn parse_keycode(&self, kc: Option<Keycode>) -> HandlerEvent {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandlerEvent {
    Quit,
    Hover(usize),
    UnHover(usize),
    Click(usize),
    Escape,
    Return,
    PushChar(u8),
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
