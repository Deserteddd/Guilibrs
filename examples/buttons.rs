extern crate guilibrs;

use std::collections::VecDeque;
use guilibrs::{GUI, Button};
const SCREENW: u32 = 600;
const SCREENH: u32 = 400;
const FONT_PATH: &'static str = "C:/Windows/Fonts/SourceCodePro-Regular.ttf";

fn main() -> Result<(), String> {
  let mut gui = GUI::new()
    .buttons(vec![
      Button::new()
        .rect((SCREENW / 3 - SCREENW / 4) as i32, SCREENH as i32 / 2 - 80, 100, 100)
        .label("Yksi")
        .color((225, 40, 70))
        .callback(1)
        .build()?,
      Button::new()
        .label("Kaksi")
        .rect((2 * SCREENW / 3 - SCREENW / 4) as i32, SCREENH as i32 / 2 - 80, 100, 100)
        .color((25, 250, 90))
        .callback(2)
        .build()?,
      Button::new()
        .label("Kolme").font(FONT_PATH)
        .rect((SCREENW - SCREENW / 4) as i32, SCREENH as i32 / 2 - 80, 100, 100)
        .color((50, 100, 255))
        .callback(3)
        .build()?,
    ])
    .size(SCREENW, SCREENH)
    .color((40, 40, 40))
    .build()?;
  
  let mut instructions: VecDeque<u8> = VecDeque::new();

  while gui.poll(&mut instructions) {
    instructions.iter().for_each(|f| match f {
      1 => println!("Yksi"),
      2 => println!("Kaksi"),
      3 => println!("Kolme"),
      _ => println!("jne."),
    });
    instructions.clear();
    gui.draw()?;
  }
  Ok(())
}

