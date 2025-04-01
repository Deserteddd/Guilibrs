# Guilibrs - Simple GUI-library using SDL2

## Library structre

### GUI - struct

The GUI-part of a Guilibrs-application is represented by the **GUI**-struct. This struct contains the entire state of the GUI and is constructed using a ***builder pattern***. Along with internal components, such as the event handler and window canvas, **GUI** contains a hashmap with **panels** and their names as key-value-pairs.

    // Simplified representation of the GUI-struct
    struct GUI {
        canvas:  Canvas<Window>,         // The window used by the GUI
        panels:  HashMap<String, Panel>, // All panels used by the GUI
        handler: EventHandler,           // Handles user input
        ..
    }

### Panel - struct

Guilibrs allows the use of multiple panels, which are the containers that hold the individual widgets. The **GUI** struct exposes methods to show and hide individual panels, which allows the creation of multi paged GUIs (see examples/color_editor).

    // Simplified representation of the Panel-struct
    struct Panel {
        name:            String,
        bounds:          Rect,            
        buttons:         []Button,
        textfields:      []TextField,
        faders:          []Fader,
        dropdownbuttons: []DropdownButton,
        ..
    }

### GuiEvent\<T\> - enum

When using Guilibrs to build applications, the GuiEvent-enum is foundational. It is used to communicate changes in the UI state to the programmer.

    enum GuiEvent<T> {
        Quit,
        ButtonPress(String, T),                     // (panel, instance of T)
        FaderUpdate(String, uint, f32),             // (panel, index, new val)
        DropdownUpdate(String, Uint, &'static str), // (panel, index, new val)
        None
    }

The generic type **T** is the type from which an instance will be returned when a button is pressed. This allows the user to, for example, create a custom enum to indicate which button was pressed:

    enum Buttons {
        Login,
        Logout,
        Exit
    }

Inside the mainloop of our program, we can poll the GUI for changes in UI. The method used for this returns an instance of GuiEvent\<T\>

### GUI::poll() - function

The poll-method of **GUI** runs the event loop of the library. Rather than hiding the execution of our program inside an App.run() -method, Guilibrs provides methods poll() and draw() that the programmer can use to advance to the next frame, when appropriate:

    main() {
        ..
        'main_loop: loop {
            match gui.poll() {
                GuiEvent::ButtonPress(panel, button) {
                    // value of 'button' is of type T, i.e. Buttons
                    match button {
                        Buttons::Logout => println!("Logging out!"),
                        ..
                    }
                    ..
                }
                ..
            }
        }
    }

Note: The poll-method blocks execution when waiting for user input.
