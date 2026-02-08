use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    game_context::{GameContext},
    renderer::Renderer,
};

pub struct App {
    pub sdl: sdl2::Sdl,
    pub game_context: GameContext,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let sdl: sdl2::Sdl = sdl2::init()?;
        let game_context: GameContext = GameContext::new()?;

        Ok(Self { sdl, game_context })
    }

    pub fn run(&mut self, renderer: &mut Renderer) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut frame_counter: u64 = 0;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::W | Keycode::Up => self.game_context.move_up(),
                        Keycode::A | Keycode::Left => self.game_context.move_left(),
                        Keycode::S | Keycode::Down => self.game_context.move_down(),
                        Keycode::D | Keycode::Right => self.game_context.move_right(),
                        Keycode::Escape => self.game_context.toggle_pause(),
                        _ => {}
                    },
                    _ => {}
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

            frame_counter += 1;
            if frame_counter.is_multiple_of(20) {
                self.game_context.next_tick();
                frame_counter = 0;
            }
            renderer.draw(self).expect("Failed to render!")
        }
    }
}
