# Note Taking Web Application written in Rust

A simplistic not taking web application written in Rust.

## Status

Currently this project is WIP

## Features

- [ ] Take notes(_duuh_)
- [ ] Authentication

## Technologies Used

- **Rust**: The main programming language used for backend development.
- **HTMX**: For dynamic content loading and interactions.
- **SASS**: Used for styling the application.
- **Tailwind CSS**: Utility-first CSS framework for styling.
- **Actix Web**: A powerful, pragmatic, and extremely fast web framework for Rust.
- **Askama**: A type-safe, compiled template engine for Rust.

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/mirage2032/htmxnotes
   ```
2. Go to the project directory:

   ```bash
   cd note-taking-app
   ```

3. Run the application:

   ```bash
   cargo run
   ```
   Alternatively if `cargo-watch` is present, you can have hot reloading by running:
   ```bash
    cargo watch -x "run" --watch "src" --watch "sass" --watch "templates"
    ```

## Building

A `build.rs` file exists in this project. It has the following functions:

### SASS

All the files under `sass` will be compiled into CSS files under `static/css`.

### Windtail

All the Windails classes that are used in the `templates` will be crated in `static/css/windtail.css`.