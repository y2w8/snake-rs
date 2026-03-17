use sdl2::{pixels::Color, rect::Rect, ttf};

use crate::{
    game_context::{GameContext, GameState, Point},
    ui::Ui,
};

pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub font: sdl2::ttf::Font<'static, 'static>,
    pub ui: Ui,
}

impl Renderer {
    pub const GRID_X_SIZE: u32 = 40;
    pub const GRID_Y_SIZE: u32 = 30;
    pub const DOT_SIZE_IN_PXS: u32 = 20;

    pub fn new(sdl: &mut sdl2::Sdl) -> Result<Self, String> {
        let video_subsystem: sdl2::VideoSubsystem = sdl.video()?;
        let window: sdl2::video::Window = video_subsystem
            .window(
                "rust-sdl2 demo",
                Self::GRID_X_SIZE * Self::DOT_SIZE_IN_PXS,
                Self::GRID_Y_SIZE * Self::DOT_SIZE_IN_PXS,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas: sdl2::render::Canvas<sdl2::video::Window> =
            window.clone().into_canvas().build().unwrap();

        let ttf = ttf::init().map_err(|e| e.to_string())?;
        let ttf_leaked: &'static sdl2::ttf::Sdl2TtfContext = Box::leak(Box::new(ttf));

        let font_data = include_bytes!("../assets/fonts/JetBrainsMonoNerdFont.ttf");

        let font = ttf_leaked
            .load_font_from_rwops(sdl2::rwops::RWops::from_bytes(font_data)?, 24)
            .map_err(|e| e.to_string())?;

        let ui: Ui = Ui::new();

        Ok(Self { canvas, font, ui })
    }

    pub fn get_center(&mut self) -> (i32, i32) {
        (
            ((Renderer::GRID_X_SIZE * Renderer::DOT_SIZE_IN_PXS) / 2) as i32,
            ((Renderer::GRID_Y_SIZE * Renderer::DOT_SIZE_IN_PXS) / 2) as i32,
        )
    }

    pub fn draw(&mut self, game_context: &GameContext) -> Result<(), String> {
        self.draw_background(game_context)?;
        self.draw_player(game_context)?;
        self.draw_food(game_context)?;

        for button in self.ui.buttons.iter_mut() {
            button.draw(&mut self.canvas, &self.font)?;
        }

        for text in self.ui.texts.iter_mut() {
            text.draw(&mut self.canvas, &self.font)?;
        }

        self.canvas.present();
        Ok(())
    }

    pub fn draw_background(&mut self, game_context: &GameContext) -> Result<(), String> {
        let bg_color = match game_context.state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::GameOver => Color::RGB(30, 30, 30),
        };

        self.canvas.set_draw_color(bg_color);
        self.canvas.clear();
        Ok(())
    }

    pub fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * Self::DOT_SIZE_IN_PXS as i32,
            y * Self::DOT_SIZE_IN_PXS as i32,
            Self::DOT_SIZE_IN_PXS,
            Self::DOT_SIZE_IN_PXS,
        ))
    }

    pub fn draw_player(&mut self, game_context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &game_context.player_position {
            self.draw_dot(point)?;
        }
        Ok(())
    }

    pub fn draw_food(&mut self, game_context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        for point in &game_context.food {
            self.draw_dot(point)?;
        }
        Ok(())
    }
}
