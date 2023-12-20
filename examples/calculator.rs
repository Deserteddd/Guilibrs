extern crate guilibrs;

use guilibrs::GUI;
use guilibrs::widget::{Button, Textbox};
use eval::{eval, to_value};
const FONT_PATH: &'static str = "C:/Windows/Fonts/vga850.fon";
const COLOR: (u8, u8, u8) = (80, 80, 80);

#[derive(Clone, Copy)]
enum Input{
  Num(char),
  Mod,
  Div,
  Mul,
  Sub,
  Sum,
  Equals,
  Clear
}

const BUTTONS: [[&'static str; 4]; 5] = [
  ["%", "/", "*", "-"], 
  ["7", "8", "9", "+"],
  ["4", "5", "6", ""],
  ["1", "2", "3", "="],
  ["", "0", ".", "c"],
];

fn main() -> Result<(), String> {
  let mut calc = setup()?;
  let mut screen = String::new();
  let mut input_buffer: Vec<Input> = vec![];
  while calc.run(&mut input_buffer) {
    if let Some(input) = input_buffer.pop(){
      match input {
        Input::Num(c) => screen.push(c),
        Input::Mod => screen.push('%'),
        Input::Div => screen.push('/'),
        Input::Sum => screen.push('+'),
        Input::Mul => screen.push('*'),
        Input::Sub => screen.push('-'),
        Input::Clear => screen.clear(),
        Input::Equals => screen = eval(&screen).unwrap().to_string(),
      }
    }
    calc.set_textbox_content(0, screen.clone());
    calc.draw()?;
  }
  Ok(())
}

fn setup() -> Result<GUI<Input>, String> {
  let mut buttons: Vec<Button<Input>> = vec![];
  for i in 0..4 {
    for j in 0..5 {
      let button = BUTTONS[j][i];
      if !button.is_empty(){
        buttons.push(Button::new()
          .rect(20 + i as i32 * 90, 90 + j as i32 * 90, 60, 60)
          .label(button)
          .color(COLOR)
          .callback(match button {
            "%" => Input::Mod,
            "/" => Input::Div,
            "*" => Input::Mul,
            "-" => Input::Sub,
            "+" => Input::Sum,
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
      Textbox::new(20, 20, 340, 40).with_content("0"),
    ])
    .size(380, 540)
    .font(FONT_PATH)
    .color((40, 40, 40))
  .build()
}
