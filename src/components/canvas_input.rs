use std::default;

use iced::{
    mouse,
    widget::{
        canvas::{self, Cache, Cursor, Geometry, Path, Program},
        Canvas,
    },
    Color, Element, Length, Point, Rectangle, Size, Theme, Vector,
};

pub struct CanvasInput {
    pub content: String,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub is_active: bool,
    pub color: (u8, u8, u8),
    pub id: i32,
    pub representation: Cache, //Not sure if scale is needed
                           // scale: f32
}

impl CanvasInput {
    pub fn new(x: f32, y: f32, id: i32) -> Self {
        Self {
            x,
            y,
            id,
            ..Default::default()
        }
    }


    pub fn view(&self, state: &(), theme: &Theme, bounds: Rectangle, cursor: Cursor ) -> Geometry {
        self.draw(state, theme, bounds, cursor)[0].clone()
    }


    pub fn update_state(&mut self, state: bool) {
        self.is_active = state;
    
        if state {
            self.color = (0, 255, 0);
        } else {
            self.color = (0, 0, 255);
        }

        self.representation.clear();
    }

    // pub fn draw(x: f32, y: f32) -> Geometry {
    //     self.
    // }

}

impl Default for CanvasInput {
    fn default() -> Self {
        Self {
            content: String::new(),
            width: 100.0,
            height: 100.0,
            x: 50.0,
            y: 50.0,
            is_active: false,
            representation: Cache::default(),
            color: (255, 0, 0),
            id: 0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GainFocus(i32),
    LostFocus(i32),
    KeyPressed(u8),
}

impl Program<Message> for CanvasInput {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let size = Size::new(self.width, self.height);
        let top_left = Point::new(self.x, self.y);
        // let bounds = Rectangle::new(top_left.clone(), size.clone());
        let component = self.representation.draw(bounds.size(), |frame| {
            frame.fill_text(format!("          {} {} {} {} {}", bounds.height, bounds.width, bounds.x, bounds.y, self.is_active));

            let color = Color::from_rgb8(
                self.color.0,
                self.color.1,
                self.color.2,
            );
            frame.fill_rectangle(top_left, size, color)
    
        });

    
        vec![component]
    }

    fn update(
            &self,
            _state: &mut Self::State,
            event: canvas::Event,
            _bounds: Rectangle,
            cursor: Cursor,
        ) -> (canvas::event::Status, Option<Message>) {
        
        let size = Size::new(self.width, self.height);
        let top_left = Point::new(self.x, self.y);
        let bounds = Rectangle::new(top_left.clone(), size.clone());

        let cursor_point = cursor.position_in(&bounds);

        match event {
            canvas::Event::Mouse(mouse) => {
                 match mouse {
                    // mouse::Event::CursorEntered => (canvas::event::Status::Captured, Some(Message::GainFocus)),
                    // mouse::Event::CursorLeft => (canvas::event::Status::Captured, Some(Message::LostFocus)),
                    
                    mouse::Event::CursorMoved { position } => {
                        if let Some(x) = cursor_point {
                            if !self.is_active {
                                (canvas::event::Status::Captured, Some(Message::GainFocus(self.id)))
                            } else {
                                (canvas::event::Status::Ignored, None)    
                            }
                        } else {
                            if self.is_active {
                                (canvas::event::Status::Captured, Some(Message::LostFocus(self.id)))
                            } else {
                                (canvas::event::Status::Ignored, None)
                            }
                        }
                    },
                    _ => (canvas::event::Status::Ignored, None)
                    // mouse::Event::ButtonPressed(_) => todo!(),
                    // mouse::Event::ButtonReleased(_) => todo!(),
                    // mouse::Event::WheelScrolled { delta } => todo!(),
                }
            },
            // canvas::Event::Touch(_) => todo!(),
            // canvas::Event::Keyboard(_) => todo!(),
            _ => (canvas::event::Status::Ignored, None)
        }

        // (canvas::event::Status::Ignored, None)
    }
}
