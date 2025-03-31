use guilibrs::{GUI, GuiEvent, Panel};
use guilibrs::widgets::{Fader, TextField, Button, TextAlign, DropdownButton};

#[derive(Debug, Clone, Copy, Default)]
enum Buttons {
    Login,
    #[default]
    Logout,
}

fn main() -> Result<(), String> {
    let login_screen = Panel::new(
        "login", (20, 20),
        vec![
            Button::new(0, 460, 340, 40)
                .label("Login")
                .color_rgb(0, 100, 20)
                .callback(Buttons::Login)
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
        vec![],
        vec![]
    );

    let mut color = (40, 40, 40);
    
    let color_editor = Panel::new(
        "editor", (20, 20),
        vec![
            Button::new(0, 460, 340, 40)
                .label("Logout")
                .color_rgb(120, 20, 20)
                .callback(Buttons::Logout),
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
        vec![
            DropdownButton::new(40, 350)
                .label("Background")
                .options(vec![
                    "Red",
                    "Blue",
                    "Green",
                ]),
        ]
    );

    let mut gui: GUI<Buttons> = GUI::new()
        .panels(&[login_screen, color_editor])
        .initial_panels(&["login"])
        .title("Demo app")
        .color(color)
        .size(380, 540)
        .build()?;


    'running: loop {
        match gui.poll() {
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
            GuiEvent::ButtonPress(panel, button) => {
                println!("Button: {:?} clicked on panel {}", button, panel);
                match button {
                    Buttons::Login => {
                        gui.hide_panel("login");
                        gui.show_panel("editor")
                    },
                    Buttons::Logout => {
                        gui.hide_panel("editor");
                        gui.show_panel("login")
                    }
                    _ => {}
                }

            },
            GuiEvent::DropdownUpdate(_, idx, option) => {
                println!("DROPDOWN UPDATE: {:?}", option);
                let color = match (idx, option) {
                    (0, "Red") => (255, 0, 0),
                    (0, "Green") => (0, 255, 0),
                    (0, "Blue") => (0, 0, 255),
                    _ => (0, 0, 0)
                };
                println!("color: {:?}", color);
                gui.set_fader_value("editor", 0, color.0 as f32);
                gui.set_fader_value("editor", 1, color.1 as f32);
                gui.set_fader_value("editor", 2, color.2 as f32);
                gui.set_backround_color(color);
                gui.set_textfield_content("editor", 0, format_rgb(color));
                gui.set_textfield_content("editor", 1, format_hex(color));
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