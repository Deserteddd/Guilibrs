extern crate guilibrs;

use eval::eval;
use guilibrs::widgets::{Button, TextField, TextAlign};
use guilibrs::{GUI, GuiEvent};

#[derive(Clone, Copy, Default)]
enum Callback {
    Num(char),
    Equals,
    #[default]
    Clear,
}    

const BUTTONS: [[&str; 4]; 5] = [
    ["%", "/", "*", "-"],
    ["7", "8", "9", "+"],
    ["4", "5", "6", "("],
    ["1", "2", "3", ")"],
    [".", "0", "c", "="],
];

fn main() -> Result<(), String> {
    let mut calc = setup()?;
    let mut running = true;
    while running {
        match calc.poll() {
            GuiEvent::None => {}
            GuiEvent::Quit => running = false,
            GuiEvent::Callback(cb) => match cb {
                Callback::Num(c) => calc.push_to_textfield(0, c as char),
                Callback::Clear => calc.clear_textfield(0),
                Callback::Equals => {
                    calc.set_textfield_content(0, evaluate(calc.textfields().nth(0).unwrap()))
                }
            },
            _ => {}
        }
        calc.draw()?;
    }

    Ok(())
}

fn evaluate(textbox: &TextField) -> String {
    let val = eval(textbox.get_content());
    if val.is_ok() {
        return val.unwrap().to_string();
    } else {
        return val.err().unwrap().to_string();
    }
}

fn setup() -> Result<GUI<Callback>, String> {
    let mut buttons: Vec<Button<Callback>> = vec![];
    for i in 0..4 {
        for j in 0..5 {
            let button = BUTTONS[j][i];
            buttons.push(
                Button::new(20 + i as i32 * 90, 90 + j as i32 * 90, 60, 60)
                    .label(button)
                    .color_rgb(
                        50 + i as u8 * 50,
                        50 + j as u8 * 50,
                        255 - 20 * (i + j) as u8,
                    )
                    .callback(match button {
                        "=" => Callback::Equals,
                        "c" => Callback::Clear,
                        _ => Callback::Num(button.chars().next().unwrap()),
                    })
            )
        }
    }

    GUI::new()
        .title("CalculatoRS")
        .textfields(vec![
            TextField::new(20, 20, 340, 40)
                .align(TextAlign::Center)
        ])
        .buttons(buttons)
        .size(380, 540)
        .color((40, 40, 40))
        .build()
}
