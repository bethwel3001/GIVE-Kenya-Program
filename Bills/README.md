# Bills Manager

A simple command-line interface I'm building to manage my bills, written in Rust.

## Stage 1: The Foundation
I started by setting up the basic project structure and core data models.
- **Data Modeling**: Created the `Bill` struct to track names and amounts.
- **In-Memory Storage**: Used a `Vec` to keep track of my bills while the app is running.
- **User Input**: Built custom helpers to handle string input and number parsing reliably.
- **Initial Features**: I can now add bills, view them in a list, and exit the app through a simple menu.

## Stage 2: Refinement & Deletion
In this stage, I focused on making the app more practical and organized.
- **Reorganization**: Renamed the project to `Bills` for simplicity.
- **Git Setup**: Configured a `.gitignore` to keep my build artifacts and research notes out of the repository.
- **Deletion**: Added the ability to remove bills I no longer need by selecting their index.
- **Improved UI**: Updated the bill list to show numbers, making it easier to pick a bill for removal.

## Stage 3: Editing & Navigation
I've just added more flexibility to how I manage my data.
- **Edit Feature**: I can now update the name or amount of any existing bill. I even made it so I can leave a field empty to keep the current value.
- **"Go Back" Logic**: I implemented a 'back' command across all prompts. This means I can change my mind at any time during an action without being forced to complete it.
- **Robust Validation**: Refined the input loops to ensure that the app doesn't crash if I enter something unexpected.

## Stage 4: Optimized Management (Learning on the Job)
Following some great tips, I've refactored the underlying data structure for better efficiency.
- **HashMap Refactor**: Switched from `Vec` to `HashMap<String, Bill>`. This makes looking up, editing, and removing bills much more direct using the bill's name as a unique key.
- **Clean Architecture**: Ensured each menu choice is handled by its own dedicated function, keeping the `main` loop clean and focused.
- **Index-Free Deletion**: No more worrying about shifting indices when removing items. I can now remove and edit bills directly by their names.

## How to Run
1. Make sure you have [Rust](https://www.rust-lang.org/) installed.
2. Navigate to the `Bills` directory.
3. Run `cargo run`.
