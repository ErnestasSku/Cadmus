use iced::{
    mouse,
    widget::{
        canvas::{self, Cache, Cursor, Geometry, Path},
        Canvas,
    },
    Color, Element, Length, Point, Rectangle, Theme, Vector,
};

pub struct EditorGraph {
    background: Cache,
    _items: Cache,
    state: Vec<Point>,
    scaling: f32,
    translation: Vector,
}

impl EditorGraph {
    pub fn view(&self) -> Element<EditorGraphMessage> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: EditorGraphMessage) {
        match message {
            EditorGraphMessage::AddBlock(point) => {
                self.state.push(point);
                self.background.clear();
            }
            EditorGraphMessage::RemoveBlock => todo!(),
            EditorGraphMessage::Translated(_) => todo!(),
            EditorGraphMessage::Scaled(_, _) => todo!(),
        }
    }
}

impl Default for EditorGraph {
    fn default() -> Self {
        EditorGraph {
            background: Cache::default(),
            _items: Cache::default(),
            state: Vec::default(),
            scaling: 1.0,
            translation: Vector::default(),
        }
    }
}

#[derive(Debug)]
pub enum EditorGraphMessage {
    // AddBlock,
    AddBlock(Point),
    RemoveBlock,
    Translated(Vector),
    Scaled(f32, Option<Vector>),
}

impl canvas::Program<EditorGraphMessage> for EditorGraph {
    type State = Interaction;

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let background = self.background.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0x4B));

            for p in self.state.iter() {
                frame.fill(&Path::circle(*p, 5.0), Color::from_rgb8(0, 255, 0));
            }

            frame.with_save(|frame| {
                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);
            })
        });

        vec![background]
    }

    fn update(
        &self,
        interaction: &mut Interaction,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (canvas::event::Status, Option<EditorGraphMessage>) {
        if let canvas::Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
            *interaction = Interaction::None;
        }

        let cursor_point = cursor.position_in(&bounds).unwrap_or(Point::ORIGIN);

        match event {
            canvas::Event::Mouse(mouse) => {
                match mouse {
                    // mouse::Event::CursorEntered => todo!(),
                    // mouse::Event::CursorLeft => todo!(),
                    // mouse::Event::CursorMoved { position } => todo!(),
                    mouse::Event::ButtonPressed(button) => {
                        match button {
                            // mouse::Button::Left => todo!(),
                            mouse::Button::Right => (
                                canvas::event::Status::Captured,
                                Some(EditorGraphMessage::AddBlock(cursor_point)),
                            ),
                            // mouse::Button::Middle => todo!(),
                            // mouse::Button::Other(_) => todo!(),
                            _ => (canvas::event::Status::Ignored, None),
                        }
                    }
                    // mouse::Event::ButtonReleased(_) => todo!(),
                    // mouse::Event::WheelScrolled { delta } => todo!(),
                    _ => (canvas::event::Status::Ignored, None),
                }
            }
            // canvas::Event::Touch(_) => todo!(),
            // canvas::Event::Keyboard(_) => todo!(),
            _ => (canvas::event::Status::Ignored, None),
        }
    }

    // fn mouse_interaction(
    //         &self,
    //         _state: &Self::State,
    //         _bounds: Rectangle,
    //         _cursor: Cursor,
    //     ) -> mouse::Interaction {
    // //  todo!()
    // }
}

pub enum Interaction {
    None,
    Panning { translation: Vector, start: Point },
    Dragging { translation: Vector, start: Point },
    // TestMsg { pos: Point }
}
impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}
