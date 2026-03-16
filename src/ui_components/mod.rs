use sdl2::{pixels::Color, rect::Rect};

use crate::{game_context::{GameEvent, Point}, ui_components::{button::Button, text::Text}};

pub mod button;
pub mod text;

pub struct Ui {
    pub buttons: Vec<Button>,
    pub texts: Vec<Text>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            buttons: vec![],
            texts: vec![],
        }
    }

    pub fn button(&mut self, x: i32, y: i32, width: u32, height: u32, text: &str, event: GameEvent) -> &mut Button {
        self.buttons.push(Button {
            rect: Rect::new(x, y, width, height),
            color: Color::RGB(0, 120, 215),
            text: text.to_string(),
            event
        });
        self.buttons.last_mut().unwrap()
    }

    pub fn text(&mut self, text: &str, color: Color, point: Point) -> &mut Text {
        self.texts.push(Text {
            text: text.to_string(),
            color,
            point

        });
        self.texts.last_mut().unwrap()
    }

    pub fn clear(&mut self) {
        self.buttons.clear();
        self.texts.clear();
    }
}
