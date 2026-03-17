use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    game_context::{GameContext, GameEvent, GameState},
    renderer::Renderer,
};

pub struct App {
    pub sdl: sdl2::Sdl,
    pub renderer: Renderer,
    pub game_context: GameContext,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let mut sdl: sdl2::Sdl = sdl2::init()?;
        let renderer: Renderer = Renderer::new(&mut sdl)?;
        let game_context: GameContext = GameContext::new()?;

        Ok(Self {
            sdl,
            renderer,
            game_context,
        })
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut frame_counter: u64 = 0;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::MouseButtonDown { x, y, .. } => {
                        let game_event = self
                            .renderer
                            .ui
                            .buttons
                            .iter()
                            .find(|btn| btn.is_clicked(x, y))
                            .map(|btn| btn.event);

                        if let Some(game_event) = game_event {
                            match game_event {
                                GameEvent::Resume => self.change_state(GameState::Playing),
                                GameEvent::Restart => {
                                    self.change_state(GameState::Playing);
                                    self.game_context.restart();
                                }
                                GameEvent::Quit => {}
                                _ => {}
                            }
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match self.game_context.state {
                        GameState::Playing => match keycode {
                            Keycode::W | Keycode::Up => self.game_context.move_up(),
                            Keycode::A | Keycode::Left => self.game_context.move_left(),
                            Keycode::S | Keycode::Down => self.game_context.move_down(),
                            Keycode::D | Keycode::Right => self.game_context.move_right(),
                            Keycode::Escape => match self.game_context.state {
                                GameState::Playing => self.change_state(GameState::Paused),
                                GameState::Paused => self.change_state(GameState::Playing),
                                _ => return,
                            },
                            _ => {}
                        },
                        GameState::Paused => {
                            if keycode == Keycode::Q {
                                break 'running;
                            }
                        }
                        GameState::GameOver => {
                            if keycode == Keycode::R {
                                self.change_state(GameState::Playing);
                                self.game_context.restart();
                            }
                        }
                    },
                    _ => {}
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

            frame_counter += 1;
            if frame_counter.is_multiple_of(20) {
                let game_event = self.game_context.next_tick();

                if let Some(game_event) = game_event {
                    match game_event {
                        GameEvent::Died => self.change_state(GameState::GameOver),
                        GameEvent::FoodEaten => {}
                        _ => {}
                    }
                    frame_counter = 0;
                }
            }
            self.renderer
                .draw(&self.game_context)
                .expect("Failed to render!")
        }
    }

    // Game State
    pub fn change_state(&mut self, new_state: GameState) {
        if self.game_context.state == new_state {
            return;
        }

        self.game_context.state = new_state;

        self.renderer.ui.clear();

        match self.game_context.state {
            GameState::Playing => {
                self.renderer.ui.clear();
            }
            GameState::Paused => {
                self.renderer.ui.clear();
                let (cx, cy) = self.renderer.get_center();
                self.renderer
                    .ui
                    .button(cx - 100, cy - 50, 200, 100, "Resume", GameEvent::Resume);
            }
            GameState::GameOver => {
                self.renderer.ui.clear();
                let (cx, cy) = self.renderer.get_center();
                self.renderer
                    .ui
                    .button(cx - 150, cy - 50, 300, 100, "Restart", GameEvent::Restart);
            }
        }
    }
}
