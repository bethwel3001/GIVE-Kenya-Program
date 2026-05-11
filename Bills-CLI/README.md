# Bills CLI

A simple command-line interface for managing bills, written in Rust.

## Stage 1 Accomplishments
- **Project Initialization**: Set up the basic Rust binary project structure.
- **Data Modeling**: Defined the `Bill` struct to hold names and amounts.
- **In-Memory Storage**: Implemented a `Vec<Bill>` to store data during the program's lifecycle.
- **User Input System**: Created robust helpers to handle string input and numeric parsing from the terminal.
- **Core Functionality**:
  - Added the ability to create new bills.
  - Implemented a view mode to list all current bills.
  - Built a persistent menu loop with exit functionality.

## How to Run
1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Navigate to the `Bills-CLI` directory.
3. Run `cargo run`.
