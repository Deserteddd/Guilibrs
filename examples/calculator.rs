extern crate guilibrs;

use guilibrs::{GUI, GuiEvent};
use guilibrs::widget::{Button, Textbox};
use eval::eval;
const FONT_PATH: &'static str = "C:/Windows/Fonts/vga850.fon";

#[derive(Clone, Copy)]
enum Input{
  Num(char),
  Equals,
  Clear
}

fn main() -> Result<(), String> {
  let mut calc = setup()?;
  let mut running = true;
  while running {
    if let Some(event) = calc.tick() {
      match event {
        GuiEvent::Quit => running = false,
        GuiEvent::KeyPress(c) => match c {
          8 => {calc.pop_from_textbox(0);},
          37 => calc.push_to_textbox(0, '%'),
          40 => calc.push_to_textbox(0, '('),
          41 => calc.push_to_textbox(0, ')'),
          42 => calc.push_to_textbox(0, '*'),
          43 => calc.push_to_textbox(0, '+'),
          45 => calc.push_to_textbox(0, '-'),
          47 => calc.push_to_textbox(0, '/'),
          13 => calc.set_textbox_content(0, evaluate(calc.textboxes().nth(0).unwrap())),
          s => if s.is_ascii_digit() {calc.push_to_textbox(0, s as char)},
        },
        GuiEvent::Custom(custom) => match custom{
          Input::Num(c) => calc.push_to_textbox(0, c),
          Input::Clear => calc.clear_textbox(0),
          Input::Equals => calc.set_textbox_content(0, evaluate(calc.textboxes().nth(0).unwrap())),
        },
      }
    }
    calc.draw()?;
  }

  Ok(())
}

fn evaluate(textbox: &Textbox) -> String {
  if let Ok(val) = eval(textbox.get_content()) {
    return val.to_string();
  }
  return textbox.get_content().to_string();
}

fn setup() -> Result<GUI<Input>, String> {
  const BUTTONS: [[&'static str; 4]; 5] = [
    ["%", "/", "*", "-"], 
    ["7", "8", "9", "+"],
    ["4", "5", "6", "("],
    ["1", "2", "3", ")"],
    [".", "0", "c", "="],
  ];

  let mut buttons: Vec<Button<Input>> = vec![];
  for i in 0..4 {
    for j in 0..5 {
      let button = BUTTONS[j][i];
      if !button.is_empty(){
        buttons.push(Button::new()
          .rect(20 + i as i32 * 90, 90 + j as i32 * 90, 60, 60)
          .label(button)
          .color((50 + i as u8 * 50, 50 + j as u8 * 50, 255-20*(i+j) as u8))
          .callback(match button {
            "=" => Input::Equals,
            "c" => Input::Clear,
            _ => Input::Num(button.chars().nth(0).unwrap(),
          )}
        ).build()?,
      )}
    }
  }

  GUI::new()
    .buttons(buttons)
    .textboxes(vec![
      Textbox::new(20, 20, 340, 40),
    ])
    .size(380, 540)
    .font(FONT_PATH)
    .color((40, 40, 40))
  .build()
}
