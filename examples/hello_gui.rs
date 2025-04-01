use guilibrs::{GuiEvent, GUI, widgets::Button};

fn main() {
    // Button location and dimensions: (x, y, width, height) in pixels. Top left == (0, 0)
    let button: Button<()> = Button::new(200, 200, 200, 200) 
        .label("Press me")
        .color_rgb(120, 20, 20);

    let mut gui: GUI<()> = GUI::new()
        .title("My App")        // Sets title of the window
        .size(600, 600)         // Sets Size of the created window (in pixels)
        .buttons(vec![button])  // Adding our button to the GUI
        .quit_on_escape()       // Makes gui.poll() return GuiEvent::Quit when Esq is pressed
        .build()                // Builds the GUI. Returns an error on failure
        .unwrap();              // We ignore error cases by unwrapping the result

    // Mainloop
    loop {
        // Poll gui for new events
        match gui.poll() { 
            // Quit is returned when window is closed or Esc is pressed
            GuiEvent::Quit => break,        

            // poll() returns this when a button is pressed.
            // Since there is only one button, we should ignore the parameters with _.
            GuiEvent::ButtonPress(_, _) => println!("Pressed"),

            // Ignore other events
            _ => {}
        };

        // Redraw and exit if any errors occur
        if let Err(e) = gui.draw() { 
            println!("Exiting with error: {}", e);
            break
        }
    }
}
