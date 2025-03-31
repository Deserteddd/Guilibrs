# Guilibrs - Simple GUI-library using SDL2

## Library structre

### GUI

The GUI-part of a Guilibrs-application is represented by the **GUI**-struct. This struct contains the entire state of the GUI and is constructed using a ***builder pattern***. Along with internal components, such as the event handler and window canvas, **GUI** contains a hashmap with **panels** and their names as key-value-pairs.

    // Simplified representation of the GUI-struct
    struct GUI {
        canvas:  Canvas<Window>,         // The window used by the GUI
        panels:  HashMap<String, Panel>, // All panels used by the GUI
        handler: EventHandler,           // Handles user input
        ..
    }

### Panel

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

### GuiEvent\<T\>

GuiEvent is an enum, that is used to relay relevant information from **GUI** to the user. 

    enum GuiEvent<T> {
        Quit,
        ButtonPress(String, T),          // (panel, instance of T)
        FaderUpdate(String, usize, f32), // (panel, fader index)
        DropdownUpdate(&'static str, usize, &'static str),
        None
    }

The generic type **T** is the type that an instance of will be returned when a button is pressed. This allows the user to create a custom enum, instead of using strings or integers to recognize, which button was pressed.
