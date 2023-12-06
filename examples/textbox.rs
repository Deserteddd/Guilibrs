extern crate guilibrs;

use guilibrs::{GUI, Button, Textbox};

const FONT_PATH: &'static str = "C:/Windows/Fonts/vga850.fon";

#[derive(Clone, Copy)]
enum Event{
  GetInput(usize)
}

fn main() -> Result<(), String> {
  let mut gui = GUI::new()
    .buttons(vec![
      Button::new()
        .rect(230, 20, 45, 30)
        .label("set")
        .color((120, 240, 120))
        .callback(Event::GetInput(0))
        .build()?,
    ])
    .textboxes(vec![
      Textbox::new(20, 20, 200, 30).clickable(),
      Textbox::new(350, 20, 200, 30),
    ])
    .size(600, 400)
    .font(FONT_PATH)
    .color((40, 40, 40))
    .build()?;
  
  let mut instructions: Vec<Event> = vec![];

  while gui.poll(&mut instructions) {
    instructions.iter().for_each(|f| match f {
      Event::GetInput(u) => println!("{}", gui.get_input(*u)),
    });
    instructions.clear();
    gui.draw()?;
  }
  Ok(())
}

