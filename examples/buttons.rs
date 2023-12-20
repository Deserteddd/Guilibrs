extern crate guilibrs;

use guilibrs::{GUI, widget::Button};
const SCREENW: u32 = 800;
const SCREENH: u32 = 600;
const FONT_PATH: &'static str = "C:/Windows/Fonts/vga850.fon";


fn main() -> Result<(), String> {
  let mut gui = GUI::new()
    .buttons(vec![
      Button::new()
        .rect((SCREENW / 3 - SCREENW / 4) as i32, SCREENH as i32 / 2 - 80, 100, 60)
        .label("Red")
        .color((225, 40, 70))
        .callback(1)
        .build()?,
      Button::new()
        .label("Green")
        .rect((2 * SCREENW / 3 - SCREENW / 4) as i32, SCREENH as i32 / 2 - 80, 100, 60)
        .color((25, 250, 90))
        .callback(2)
        .build()?,
      Button::new()
        .label("Blue")
        .rect((SCREENW - SCREENW / 4) as i32, SCREENH as i32 / 2 - 80, 100, 60)
        .color((50, 100, 255))
        .callback(3)
        .build()?,
    ])
    .size(SCREENW, SCREENH)
    .font(FONT_PATH)
    .color((40, 40, 40))
    .build()?;
  
  let mut instructions: Vec<u8> = Vec::new();

  while gui.run(&mut instructions) {
    instructions.iter().for_each(|f| match f {
      0 => println!("Default functionality"),
      1 => println!("Clicked red"),
      2 => println!("Clicked green"),
      3 => println!("Clicked blue"),
      n => panic!("Received undeclared instruction from GUI: {n}"),
    });
    instructions.clear();
    gui.draw()?;
  }
  Ok(())
}

