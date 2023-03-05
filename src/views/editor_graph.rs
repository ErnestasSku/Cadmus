use iced::{
    mouse,
    widget::{
        canvas::{self, Cache, Cursor, Geometry, Path},
        Canvas,
    },
    Color, Element, Length, Point, Rectangle, Theme, Vector,
};

use crate::components::canvas_input::{self, CanvasInput};

pub struct EditorGraph {
    background: Cache,
    _items: Cache,
    state: Vec<Point>,
    scaling: f32,
    translation: Vector,

    canvas_inputs_test: Vec<CanvasInput>,
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
                // self.state.push(point);
                self.canvas_inputs_test.push(CanvasInput::new(
                    point.x,
                    point.y,
                    self.canvas_inputs_test.len() as i32 + 1,
                ));
                self.background.clear();
            }
            EditorGraphMessage::RemoveBlock => todo!(),
            EditorGraphMessage::Translated(_) => todo!(),
            EditorGraphMessage::Scaled(_, _) => todo!(),
            EditorGraphMessage::Input(msg) => match msg {
                canvas_input::Message::GainFocus(id) => {
                    self.canvas_inputs_test
                        .iter_mut()
                        .find(|x| x.id == id)
                        .and_then(|x| Some(x.update_state(true)));
                }
                canvas_input::Message::LostFocus(id) => {
                    self.canvas_inputs_test
                        .iter_mut()
                        .find(|x| x.id == id)
                        .and_then(|x| Some(x.update_state(false)));
                }
                canvas_input::Message::KeyPressed(_) => todo!(),
            },
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

            canvas_inputs_test: Vec::default(), // canvas_inputs_test: CanvasInput::
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EditorGraphMessage {
    // AddBlock,
    AddBlock(Point),
    RemoveBlock,
    Translated(Vector),
    Scaled(f32, Option<Vector>),

    Input(canvas_input::Message),
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

            //TEST

            // for i in self.canvas_inputs_test.iter() {
            //     i.view();
            // }

            //

            frame.fill_text(format!("{} ", self.canvas_inputs_test.len()));

            frame.with_save(|frame| {
                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);
            })
        });

        //TEST
        // let a = CanvasInput::draw(&CanvasInput::new(50.0, 50.0), &(), _theme, bounds, _cursor);

        let mut a = vec![];
        for i in self.canvas_inputs_test.iter() {
            a.push(i.view(&(), &_theme, bounds, _cursor.clone()))
        }

        //

        // vec![background].iter().chain(a.iter()).collect()

        let mut ret = vec![background];
        for i in a.iter() {
            ret.push(i.clone());
        }
        // let b: Vec<Geometry> = ret.iter_mut().chain(a.iter_mut()).collect();

        ret
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

        let a: Vec<(canvas::event::Status, Option<EditorGraphMessage>)> = self
            .canvas_inputs_test
            .iter()
            .map(|x| x.update(&mut (), event.clone(), bounds.clone(), cursor.clone()))
            .filter(|x| x.0 != canvas::event::Status::Ignored)
            .map(|(s, m)| {
                let new = m.map(|x| EditorGraphMessage::Input(x));

                (s, new)
            })
            .collect();

        if !a.is_empty() {
            return a[0];
        }

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
