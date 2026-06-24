# Iron-Shell

Iron-Shell is a custom, modern command-line shell built in Rust with a graphical user interface (GUI) powered by `egui` and `eframe`. It features a sleek, translucent dark-themed window with native Windows blur effects, providing a unique blend of terminal utility and modern UI aesthetics.

## Features

- **Modern GUI Shell**: Runs as a standalone GUI application rather than inside a standard terminal emulator.
- **Sleek Aesthetics**: Custom transparent background, borderless window, and `window-vibrancy` blur effects (Windows native).
- **Built-in Commands**: Includes a suite of easy-to-use file management built-ins:
  - `cd [path]` - Change directory (defaults to home).
  - `create folder <name>` / `create file <name>` - Create files and directories.
  - `delete folder <name>` / `delete file <name>` - Delete files and directories.
  - `list folder [path]` - List contents of a directory.
  - `read file <name>` - Read and display file contents.
  - `copy file|folder <src> <dest>` - Copy files or directories.
  - `move file|folder <src> <dest>` / `rename file|folder <src> <dest>` - Move or rename files and directories.
  - `hi` - Greets the current user!
  - `exit` - Close the shell.
- **Pipeline Execution**: Supports executing external commands and basic shell pipelines.

## Technologies Used

- [Rust](https://www.rust-lang.org/) (Edition 2024)
- [egui](https://github.com/emilk/egui) & [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) for the graphical interface.
- [window-vibrancy](https://github.com/tauri-apps/window-vibrancy) for native OS blurring effects.
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) & [rustyline](https://github.com/kkawakam/rustyline)

## Running Locally

Make sure you have Rust and Cargo installed, then run:

```bash
cargo run
```
