extern crate guilibrs;

use guilibrs::{GUI, GuiEvent, Panel, TextAlign};
use guilibrs::widgets::{Fader, TextField, Button};

fn main() -> Result<(), String> {
    let login_screen = Panel::new(
        vec![
            Button::new(20, 480, 340, 40)
                .label("Login")
                .color_rgb(20, 120, 20)
                .callback(1)
        ],
        vec![
            TextField::new(20, 20, 340, 40)
                .label("Username")
                .clickable(),
            TextField::new(20, 80, 340, 40)
                .label("Password")
                .password()
                .align(TextAlign::Left(0))
                .clickable()
        ],
        vec![]
    );

    let mut color = (40, 40, 40);
    
    let mut color_editor = Panel::new(
        vec![
            Button::new(20, 480, 340, 40)
                .label("Logout")
                .color_rgb(120, 20, 20)
                .callback(1),
        ],
        vec![
            TextField::new(90, 20, 200, 40)
                .label("RGB")
                .transparent()
                .align(TextAlign::Center)
        ],
        vec![
            Fader::new(20, 140, 340),
            Fader::new(20, 200, 340),
            Fader::new(20, 260, 340)
        ]
    );
    color_editor.textfields[0].set_content("40, 40, 40".to_string());


    let mut gui: GUI<u8> = GUI::new()
        .panel(login_screen)
        .title("TextFields")
        .color(color)
        .size(380, 540)
        .build()?;

    'running: loop {
        match gui.poll() {
            GuiEvent::None => {}
            GuiEvent::Quit => break 'running,
            GuiEvent::Callback(u) => match u {
                1 => { 
                    gui.textfields().for_each(|tf| println!("{}", tf));
                    color_editor = gui.swap_panel(color_editor); 
                },
                2 => println!("Doing stuff..."),
                _ => {}
            }
            GuiEvent::FaderUpdate(u, f) => {
                match u {
                    0 => color.0 = (f * 255.0) as u8,
                    1 => color.1 = (f * 255.0) as u8,
                    2 => color.2 = (f * 255.0) as u8,
                    _ => {}
                };
                gui.set_textfield_content(0, format!("{}, {}, {}", color.0, color.1, color.2));
                gui.set_backround_color(color);
                println!("Fader {} moved to {}", u, f);
            }
        }
        gui.draw()?;
    }
    Ok(())
}