// use std::path::Path;
use iced::{Sandbox, Settings};

mod views;
mod components;

fn main() -> iced::Result {

    views::editor::Editor::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}
