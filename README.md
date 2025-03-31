# Guilibrs - Simple GUI-library using SDL2

## Overview

The GUI-part of a Guilibrs-application is built on top of the **GUI**-struct. This struct contains the entire state of the GUI and is constructed using a builder pattern.

Along with internal components, such as the event handler and window canvas, **GUI** contains a hashmap with **panels** and their names as key-value-pairs. Each **panel** in turn holds the widgets that live inside that panel.
## Widgets

### Button

Buttons 