use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf, video::Window};

use crate::game_context::Point;

pub struct Text {
    pub text: String,
    pub color: Color,
    pub point: Point,
}

impl Text {
    pub fn new(text: &str, color: Color, point: Point) -> Self {
        Self {
            text: text.to_string(),
            color,
            point,
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        font: &ttf::Font<'static, 'static>,
    ) -> Result<(), String> {
        let surface = font
            .render(&self.text)
            .blended(self.color)
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = Rect::new(
            self.point.0,
            self.point.1,
            surface.width(),
            surface.height(),
        );
        canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
}
