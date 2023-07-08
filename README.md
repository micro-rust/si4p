# defmthost

`defmthost` is a top level GUI application to parse `defmt` logging frames over USB transport.

[`defmt`](https://github.com/knurling-rs/defmt) is a highly efficient logging framework targetting resource constrained devices like microcontrollers.

This API is formed by two parts, host side tooling (this application) and microncontroller side tooling ([`defmtusb`](https://github.com/micro-rust/defmtusb) or `defmtuart` (WIP)).

## Installation

Simply run the next command to install the host side application with the graphical interface

```
cargo install defmthost --features gui
```

or with the terminal interface (WIP)

```
cargo install defmthost --features tui
```

## GUI - Getting started

![Default GUI](/res/DemoDEFMThost.png "Default GUI Example")

To get started with the GUI, select the ELF file containing the `defmt` data. This will open an OS native dialog to select the file.

To select which USB device to use, select it using the dropdown on the right of the screen. These dropdowns allow the selection of a specific configuration, interface and enpoint, although only IN Bulk endpoints in CDC ACM interfaces are currently allowed.

![Selection Dropdown](/res/SelectionDropdown.png "Selection Dropdown")

## Current status

### What works?

 - USB device selection (max. one device active per application instance)
 
 - Connection over USB to `defmtusb` devices

 - Colored console logs

 - Console messages filtering based on source (host / target) and level (trace, debug, info, warn, error)

 - ELF file selection

### Planned improvements

 - ELF binary file reloading

 - Better / More resilient USB connectivity

 - Dinamic stylesheets and themes (with hot reload)

 - More ergonomic UI / UX
