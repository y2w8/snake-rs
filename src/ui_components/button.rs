use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf;
use sdl2::video::Window;

use crate::game_context::GameEvent;

pub struct Button {
    pub rect: Rect,
    pub color: Color,
    pub text: String,
    pub event: GameEvent
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str, event: GameEvent) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            color: Color::RGB(0, 120, 215),
            text: text.to_string(),
            event,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, font: &ttf::Font<'static, 'static>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect)?;

        let surface = font
            .render(&self.text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let text_rect = Rect::new(
            self.rect.x() + (self.rect.width() as i32 - width as i32) / 2,
            self.rect.y() + (self.rect.height() as i32 - height as i32) / 2,
            width,
            height,
        );

        canvas.copy(&texture, None, Some(text_rect))?;
        Ok(())
    }

    pub fn is_clicked(&self, mouse_x: i32, mouse_y: i32) -> bool {
        self.rect.contains_point((mouse_x, mouse_y))
    }
}
