use std::ops::Add;
use rand::Rng;

use crate::{game_context, renderer::Renderer};

pub enum GameState {
    Playing,
    Paused,
}

#[derive(Copy, Clone, PartialEq)] // أضف هذي السطور
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
            rng.random_range(0..Renderer::GRID_X_SIZE as i32)
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
        if let GameState::Paused = self.state {
            return;
        }

        let back_position = *self.player_position.last().unwrap();
        let next_head_position = self.next_head_position(self.player_direction);


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
        loop {
            let new_food = Point::random();
            if !self.player_position.contains(&new_food) {
                self.food.push(new_food);
                break;
            }
        }
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
        match direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        }
    }


    // Game State
    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        }
    }
}
