use iced::{Element, Length, Sandbox, Theme};

use iced::widget::{column, container};

use super::editor_graph::*;

pub struct Editor {
    editor_graph: EditorGraph,
    // file_path: Option<String>, //todo
}

#[derive(Debug)]
pub enum EditorMessage {
    Graph(EditorGraphMessage),
}

impl Sandbox for Editor {
    type Message = EditorMessage;

    fn new() -> Self {
        Self {
            editor_graph: EditorGraph::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Cadmus")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            EditorMessage::Graph(message) => {
                self.editor_graph.update(message);
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = column![self
            .editor_graph
            .view()
            .map(move |message| EditorMessage::Graph(message))];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }
}
