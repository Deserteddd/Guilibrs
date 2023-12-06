use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::{EventPump, Sdl};

pub struct EventHandler {
  pump: EventPump,
  hovered: Option<usize>,
  lmb_down: (bool, i32, i32),
}

impl EventHandler {
  pub fn new(context: &Sdl) -> Result<EventHandler, String> {
    let pump = context.event_pump()?;
    Ok(EventHandler {
      pump: pump,
      hovered: None,
      lmb_down: (false, 0, 0),
    })
  }
  pub fn poll(&mut self, bounds: &[Rect]) -> Vec<HInstruction>{
    let mut buffer: Vec<HInstruction> = Vec::new();
    self.pump.poll_iter().for_each(|event| match event {
      Event::Quit {..} => buffer.push(HInstruction::Quit),
      Event::KeyDown { keycode, .. } => match keycode {
        Some(Keycode::Escape) => buffer.push(HInstruction::Escape),
        Some(Keycode::Return) => buffer.push(HInstruction::Return),
        Some(key) => {println!("{:?}", key); buffer.push(HInstruction::Keypress((key as u8) as char))},
        _ => {},
      },
      Event::MouseMotion {x, y, .. } => {
        let mut hover_instruction: Option<HInstruction> = None;
        if let Some(idx) = self.hovered {
          if !in_bounds(&bounds[idx], x, y) {
            hover_instruction = Some(HInstruction::UnHover(idx))
          } 
        } else {
          for (idx, b) in bounds.iter().enumerate() {
            if in_bounds(b, x, y) {
              hover_instruction = Some(HInstruction::Hover(idx));
            }
          }
        }
        match hover_instruction {
          Some(HInstruction::Hover(idx)) => {
            self.hovered = Some(idx);
            buffer.push(HInstruction::Hover(idx))
          },
          Some(HInstruction::UnHover(idx)) => {
            self.hovered = None;
            buffer.push(HInstruction::UnHover(idx))
          },
          Some(_) => {},
          None => {},
        }
      },
      Event::MouseButtonDown {mouse_btn, x, y, ..} => {
        if mouse_btn == MouseButton::Left {
          self.lmb_down = (true, x, y);
        }
      },
      Event::MouseButtonUp {mouse_btn, ..} => {
        if mouse_btn == MouseButton::Left {
          self.lmb_down.0 = false;
          if let Some(idx) = self.hovered{
            if in_bounds(&bounds[idx], self.lmb_down.1, self.lmb_down.2) {
              buffer.push(HInstruction::Click(idx))
            }
          }
        }
      },
      _ => {},
    });
    buffer
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HInstruction {
  Quit,
  Hover(usize),
  UnHover(usize),
  Click(usize),
  Keypress(char),
  Escape,
  Return
}

fn in_bounds(rect: &Rect, x: i32, y: i32) -> bool {
  if x >= rect.x && x <= rect.x + rect.w &&
      y >= rect.y && y <= rect.y + rect.h 
  {
    return true;
  }
  false
}