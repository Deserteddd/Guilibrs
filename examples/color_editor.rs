use guilibrs::{GUI, GuiEvent, Panel};
use guilibrs::widgets::{Fader, TextField, Button, TextAlign, DropdownButton};

fn main() -> Result<(), String> {
    let login_screen = Panel::new(
        "login", (20, 20),
        vec![
            Button::new(0, 460, 340, 40)
                .label("Login")
                .color_rgb(0, 100, 20)
                .callback(1)
        ],
        vec![
            TextField::new(0, 0, 340, 40)
                .label("Username")
                .align(TextAlign::Left(10))
                .clickable(),
            TextField::new(0, 60, 340, 40)
                .label("Password")
                .password()
                .align(TextAlign::Left(10))
                .clickable()
        ],
        vec![
            Fader::new(40, 400, 280)
                .vertical()
                .range(0., 255.)
                .display_on_hover(),
        ],
        vec![
            DropdownButton::new(140, 150)
                .options(vec![
                    "Moi",
                    "Mitä",
                    "Kuuluu",
                    "Kukku",
                    "Luuruu"
                ])
        ]
    );

    let mut color = (40, 40, 40);
    
    let color_editor = Panel::new(
        "editor", (400, 20),
        vec![
            Button::new(0, 460, 340, 40)
                .label("Logout")
                .color_rgb(120, 20, 20)
                .callback(1),
        ],
        vec![
            TextField::new(50, 0, 280, 40)
                .transparent()
                .content(&format_rgb(color))
                .align(TextAlign::Center),
            TextField::new(50, 40, 280, 40)
                .transparent()
                .content(&format_hex(color))
                .align(TextAlign::Center),
        ],
        vec![
            Fader::new(0, 180, 340)
                .range(0., 255.)
                .initial(40.),
            Fader::new(0, 240, 340)
                .range(0., 255.)
                .initial(40.),
            Fader::new(0, 300, 340)
                .range(0., 255.)
                .initial(40.)
        ],
        vec![]
    );

    let mut gui: GUI<u32> = GUI::new()
        .panels(&[login_screen, color_editor])
        .title("Demo app")
        .color(color)
        .size(760, 540)
        .build();


    'running: loop {
        match gui.poll().unwrap() {
            GuiEvent::None => {}
            GuiEvent::Quit => break 'running,
            GuiEvent::FaderUpdate(panel, u, f) => {
                println!("Fader {} on panel {} changed to {}", u, panel, f);
                match (panel, u) {
                    ("editor", 0) => color.0 = f as u8,
                    ("editor", 1) => color.1 = f as u8,
                    ("editor", 2) => color.2 = f as u8,
                    _ => {}
                };
                gui.set_textfield_content("editor", 0, format_rgb(color));
                gui.set_textfield_content("editor", 1, format_hex(color));
                gui.set_backround_color(color);
            },
            GuiEvent::Callback(panel, num) => {
                println!("Clicked button {} on panel {}", num, panel);
            },
            GuiEvent::DropdownUpdate(panel, u, option) => {
                println!("Dropdown menu {u} on panel {panel} updated to: {option}");
            }
        }
        // gui.draw().map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn format_hex(color: (u8, u8, u8)) -> String {
    format!("HEX: #{:02x}{:02x}{:02x}", color.0, color.1, color.2)
}

fn format_rgb(color: (u8, u8, u8)) -> String {
    format!("RGB: {}, {}, {}", color.0, color.1, color.2)
}