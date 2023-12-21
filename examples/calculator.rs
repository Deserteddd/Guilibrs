extern crate guilibrs;

use guilibrs::{GUI, GuiEvent};
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



fn main() -> Result<(), String> {
  let mut calc = setup()?;
  let mut screen = String::new();
  let mut running = true;
  while running {
    if let Some(event) = calc.tick() {
      match event {
        GuiEvent::Quit => running = false,
        GuiEvent::Custom(custom) => match custom{
          Input::Num(c) => calc.push_to_textbox(0, c),
          Input::Mod => calc.push_to_textbox(0, '%'),
          Input::Div => calc.push_to_textbox(0, '/'),
          Input::Sum => calc.push_to_textbox(0, '+'),
          Input::Mul => calc.push_to_textbox(0, '*'),
          Input::Sub => calc.push_to_textbox(0, '-'),
          Input::Clear => calc.clear_textbox(0),
          Input::Equals => if let Ok(v) = eval(calc.textboxes().nth(0).unwrap().get_content()){
            calc.set_textbox_content(0, v.to_string());
          },
        }
      }
    }
    //calc.set_textbox_content(0, screen.clone());
    calc.draw()?;
  }

  Ok(())
}

fn setup() -> Result<GUI<Input>, String> {
  const BUTTONS: [[&'static str; 4]; 5] = [
    ["%", "/", "*", "-"], 
    ["7", "8", "9", "+"],
    ["4", "5", "6", ""],
    ["1", "2", "3", "="],
    ["", "0", ".", "c"],
  ];

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
      Textbox::new(20, 20, 340, 40),
    ])
    .size(380, 540)
    .font(FONT_PATH)
    .color((40, 40, 40))
  .build()
}
