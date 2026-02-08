use sdl2::{pixels::Color, rect::Rect};

use crate::{
    app::App,
    game_context::{GameContext, GameState, Point},
};


pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Renderer {
    pub const GRID_X_SIZE: u32 = 40;
    pub const GRID_Y_SIZE: u32 = 30;
    pub const DOT_SIZE_IN_PXS: u32 = 20;

    pub fn new(app: &mut App) -> Result<Self, String> {
        let video_subsystem: sdl2::VideoSubsystem = app.sdl.video()?;
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

        Ok(Self {
            canvas,
        })
    }

    pub fn draw(&mut self, app: &mut App) -> Result<(), String> {
        self.draw_background(&app.game_context);
        let _ = self.draw_player(&app.game_context);
        let _ = self.draw_food(&app.game_context);
        self.canvas.present();
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

    pub fn draw_background(&mut self, game_context: &GameContext) {
        let bg_color = match game_context.state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::Over => Color::RGB(30, 30, 30),
        };

        self.canvas.set_draw_color(bg_color);
        self.canvas.clear();
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
