// use std::path::Path;
use iced::{Sandbox, Settings};

mod views;

fn main() -> iced::Result {

    views::editor::Editor::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}
