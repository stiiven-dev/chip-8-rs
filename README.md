# CHIP-8 Emulator in Rust

A CHIP-8 emulator written in Rust as a learning project to explore emulator development, low-level programming, and Rust.

---

## Overview

CHIP-8 is a simple interpreted programming language originally developed in the 1970s for hobbyist computers. Despite its simplicity, it provides an excellent introduction to emulator development because it includes many of the core concepts found in more complex systems.

This project aims to build a clean and modular CHIP-8 emulator while learning Rust and emulator architecture.

---

## Features

Current implementation:

- CPU fetch-decode-execute cycle
- 4 KB memory
- 16 general-purpose registers
- Index register
- Program counter
- Stack and subroutines
- Delay and sound timers
- ROM loading
- 64×32 monochrome display
- Keyboard input

---

## Installation

### Prerequisites

- Rust (latest stable)
- Cargo

Clone the repository:

```bash
git clone https://github.com/yourusername/chip8-rust.git

cd chip8-rust
```

Build the project:

```bash
cargo build --release
```
---

## Usage

Run a ROM:

```bash
cargo run roms/PONG.ch8
```

### Keyboard Mapping

| CHIP-8 | Keyboard |
|--------|----------|
| 1 2 3 C | 1 2 3 4 |
| 4 5 6 D | Q W E R |
| 7 8 9 E | A S D F |
| A 0 B F | Z X C V |

---

## Dependencies

Major crates used:

- SDL2 (graphics, keyboard, audio)
- rand

---

## Project Structure

```
src/
└── main.rs

roms/
tests/
Cargo.toml
README.md
```

---

## Contributing

This project is primarily a learning exercise, but contributions, suggestions, and bug reports are welcome.

Possible areas for improvement:

- More opcode tests
- Performance optimizations
- Super CHIP support
- Better documentation

---

## License

This project is licensed under the Unlicense License.

---

## Learning Resources

### Rust

- The Rust Programming Language
- Rust By Example
- Rustlings

### CHIP-8

- [Cowgod's Chip-8 Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [CHIP-8 test ROMs](https://github.com/Timendus/chip8-test-suite)

---

## Acknowledgements

Thanks to the Rust community and the emulator development community for the excellent documentation and learning resources that made this project possible.