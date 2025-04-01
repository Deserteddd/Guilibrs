use eval::eval;
use guilibrs::widgets::{Button, TextField, TextAlign};
use guilibrs::{GuiEvent, GUI};

#[derive(Clone, Copy, Default)]
enum Buttons {
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
            GuiEvent::ButtonPress(_, button) => match button {
                Buttons::Num(c) => calc.push_to_textfield(0, c as char),
                Buttons::Clear => calc.clear_textfield(0),
                Buttons::Equals => {
                    calc.set_textfield_content(
                        0, evaluate(calc.textfields().nth(0).unwrap())
                    )
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

fn setup() -> Result<GUI<Buttons>, String> {
    let mut buttons: Vec<Button<Buttons>> = vec![];
    for i in 0..4 {
        for j in 0..5 {
            let button = BUTTONS[j][i];
            buttons.push(
                Button::new(i as i32 * 90 + 20, 70 + j as i32 * 90 + 20, 60, 60)
                    .label(button)
                    .color_rgb(
                        50 + i as u8 * 50,
                        50 + j as u8 * 50,
                        255 - 20 * (i + j) as u8,
                    )
                    .callback(match button {
                        "=" => Buttons::Equals,
                        "c" => Buttons::Clear,
                        _ => Buttons::Num(button.chars().next().unwrap()),
                    })
            )
        }
    }

    GUI::new()
        .title("CalculatoRS")
        .quit_on_escape()
        .buttons(buttons)
        .textfields(vec![
            TextField::new(20, 20, 340, 40)
                .align(TextAlign::Center)
                .writeable()
        ])
        .size(380, 540)
        .color((40, 40, 40))
        .build()
}
