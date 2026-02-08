use rand::Rng;
use std::ops::Add;

use crate::renderer::Renderer;

#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    Over,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Point(
            rng.random_range(0..Renderer::GRID_X_SIZE as i32),
            rng.random_range(0..Renderer::GRID_X_SIZE as i32),
        )
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Vec<Point>,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            food: vec![Point(3, 3)],
            state: GameState::Playing,
        })
    }

    pub fn next_tick(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        let back_position = *self.player_position.last().unwrap();
        let next_head_position = self.next_head_position(self.player_direction);

        if self.player_position.contains(&next_head_position) {
            self.state = GameState::Over;
            return;
        }
        self.player_position.insert(0, next_head_position);

        if self.food.contains(&next_head_position) {
            self.player_position.push(back_position);

            self.food.retain(|p| *p != next_head_position);

            self.generate_food();
        } else {
            self.player_position.pop();
        }
    }

    // Food
    fn generate_food(&mut self) {
        let empty_points: Vec<Point> = (0..Renderer::GRID_Y_SIZE)
            .flat_map(|y| (0..Renderer::GRID_X_SIZE).map(move |x| Point(x as i32, y as i32)))
            .filter(|p| !self.player_position.contains(p) && !self.food.contains(p))
            .collect();
        let mut rng = rand::rng();
        if empty_points.is_empty() { return };
        self.food.push(empty_points[rng.random_range(0..empty_points.len() - 1)])
    }

    // Player
    pub fn move_up(&mut self) {
        let next_head_position = self.next_head_position(PlayerDirection::Up);
        if self.player_position.contains(&next_head_position) {
            return;
        }

        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        let next_head_position = self.next_head_position(PlayerDirection::Down);
        if self.player_position.contains(&next_head_position) {
            return;
        }

        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        let next_head_position = self.next_head_position(PlayerDirection::Right);
        if self.player_position.contains(&next_head_position) {
            return;
        }

        self.player_direction = PlayerDirection::Right;
    }

    pub fn move_left(&mut self) {
        let next_head_position = self.next_head_position(PlayerDirection::Left);
        if self.player_position.contains(&next_head_position) {
            return;
        }

        self.player_direction = PlayerDirection::Left;
    }

    pub fn next_head_position(&mut self, direction: PlayerDirection) -> Point {
        let head_position = self.player_position.first().unwrap();
        let next_head_position = match direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };
        match next_head_position {
            mut head if head.0.is_negative() => {
                head.0 = Renderer::GRID_X_SIZE as i32 - 1;
                head
            }

            mut head if head.0 > Renderer::GRID_X_SIZE as i32 => {
                head.0 = 0;
                head
            }

            mut head if head.1.is_negative() => {
                head.1 = Renderer::GRID_Y_SIZE as i32 - 1;
                head
            }

            mut head if head.1 > Renderer::GRID_Y_SIZE as i32 => {
                head.1 = 0;
                head
            }
            _ => next_head_position,
        }
    }

    // Game State
    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            _ => return,
        }
    }

    pub fn restart(&self) {
        todo!()
    }
}
