extern crate guilibrs;
use guilibrs::{GUI, GuiEvent, Panel};
use guilibrs::widgets::{Fader, TextField, Button, TextAlign};


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
                .align(TextAlign::Left(10))
                .clickable(),
            TextField::new(20, 80, 340, 40)
                .label("Password")
                .password()
                .align(TextAlign::Left(10))
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
            TextField::new(70, 20, 280, 40)
                .transparent()
                .content(&format_rgb(color))
                .align(TextAlign::Left(0)),
            TextField::new(70, 60, 280, 40)
                .transparent()
                .content(&format_hex(color))
                .align(TextAlign::Left(0)),
        ],
        vec![
            Fader::new(20, 200, 340)
                .range(0., 255.)
                .initial(40.),
            Fader::new(20, 260, 340)
                .range(0., 255.)
                .initial(40.),
            Fader::new(20, 320, 340)
                .range(0., 255.)
                .initial(40.)
        ]
    );

    let mut gui: GUI<u32> = GUI::new()
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
                _ => {}
            }
            GuiEvent::FaderUpdate(u, f) => {
                match u {
                    0 => color.0 = f as u8,
                    1 => color.1 = f as u8,
                    2 => color.2 = f as u8,
                    _ => {}
                };
                gui.set_textfield_content(0, format_rgb(color));
                gui.set_textfield_content(1, format_hex(color));
                gui.set_backround_color(color);
            }
        }
        gui.draw()?;
    }
    Ok(())
}

fn format_hex(color: (u8, u8, u8)) -> String {
    format!("HEX: #{:02x}{:02x}{:02x}", color.0, color.1, color.2)
}

fn format_rgb(color: (u8, u8, u8)) -> String {
    format!("RGB: {}, {}, {}", color.0, color.1, color.2)
}