use std::{collections::VecDeque, thread::current};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ruscii::app::{App, Config, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

#[derive(Clone)]
enum Piece {
    L,
    N,
    Square,
}

impl Piece {
    pub fn get_tiles(&self) -> Vec<Vec2> {
        match self {
            Piece::L => {
                vec![
                    Vec2::xy(2, 1),
                    Vec2::xy(2, 2),
                    Vec2::xy(2, 3),
                    Vec2::xy(3, 3),
                ]
            }
            Piece::N => {
                vec![
                    Vec2::xy(3, 1),
                    Vec2::xy(2, 2),
                    Vec2::xy(3, 2),
                    Vec2::xy(2, 3),
                ]
            }
            Piece::Square => {
                vec![
                    Vec2::xy(2, 2),
                    Vec2::xy(2, 3),
                    Vec2::xy(3, 3),
                    Vec2::xy(3, 2),
                ]
            }
        }
    }
}

impl Distribution<Piece> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Piece {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) {
            // rand 0.8
            0 => Piece::L,
            1 => Piece::N,
            _ => Piece::Square,
        }
    }
}

#[derive(Clone)]
struct CurrentPiece {
    piece_type: Piece,
    location: Vec2,
    matrix: Vec<Vec<u8>>,
}

impl CurrentPiece {
    pub fn new(location: Vec2) -> Self {
        let piece_type: Piece = rand::random();
        let matrix: Vec<Vec<u8>> = (0..5)
            .map(|y| {
                (0..5)
                    .map(|x| {
                        if piece_type.get_tiles().contains(&Vec2::xy(x, y)) {
                            1
                        } else {
                            0
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            piece_type,
            location,
            matrix,
        }
    }
    pub fn relative_tiles(&self) -> Vec<Vec2> {
        let mut tiles: Vec<Vec2> = Vec::new();

        for x in 0..5 {
            for y in 0..5 {
                if self.matrix[y][x] == 1 {
                    tiles.push(Vec2::xy(x, y));
                }
            }
        }

        let new_tiles = tiles
            .iter()
            .map(|&t| Vec2::xy(t.x + self.location.x, t.y + self.location.y))
            .collect();

        new_tiles
    }
    pub fn rotate(&mut self) {
        self.transpose();
        self.reverse_rows();
    }

    fn transpose(&mut self) {
        let mut new_matrix = vec![vec![0; 5]; 5];
        for y in 0..5 {
            for x in 0..5 {
                new_matrix[y][x] = self.matrix[x][y];
            }
        }
        self.matrix = new_matrix;
    }

    fn reverse_rows(&mut self) {
        for y in 0..5 {
            self.matrix[y] = self.matrix[y].iter().rev().cloned().collect();
        }
    }
}

struct GameState {
    pub dimension: Vec2,
    pub tiles: Vec<Vec2>,
    pub current_piece: CurrentPiece,
    pub last_update: usize,
    pub last_rotate: usize,
    pub last_input: (usize, i32), // frame, direction
    pub drop_speed: usize,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        GameState {
            dimension,
            tiles: Vec::new(),
            last_update: 0,
            last_input: (0, 0),
            last_rotate: 0,
            drop_speed: 20,
            current_piece: CurrentPiece::new(Vec2::xy(dimension.x / 2 - 3, -4)),
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.last_update + self.drop_speed < frame {
            self.last_update = frame;
            if self.will_collide(Vec2::xy(0, 1), false) {
                for tile in self.current_piece.relative_tiles() {
                    self.tiles.push(tile.clone())
                }
                self.current_piece = CurrentPiece::new(Vec2::xy(self.dimension.x / 2 - 3, -4))
            } else {
                self.current_piece.location.y += 1;
            }
        }
    }

    pub fn rotate_piece(&mut self) {
        self.current_piece.rotate();
    }

    pub fn will_collide(&mut self, movement: Vec2, walls: bool) -> bool {
        let mut future_piece = self.current_piece.clone();
        future_piece.location.x += movement.x;
        future_piece.location.y += movement.y;
        let mut collision = false;
        for piece_tile in future_piece.relative_tiles() {
            if walls && (piece_tile.x < 0 || piece_tile.x > self.dimension.x) {
                collision = true;
                break;
            }
            if piece_tile.y > self.dimension.y {
                collision = true;
                break;
            }
            for tile in &self.tiles {
                if *tile == piece_tile {
                    collision = true;
                    break;
                }
            }
        }
        collision
    }

    pub fn tile_move_x(&mut self, displacement: i32, frame: usize) {
        if self.last_input.0 + 4 < frame || self.last_input.1 != displacement {
            self.last_input = (frame, displacement);
            if !self.will_collide(Vec2::xy(displacement, 0), true) {
                self.current_piece.location.x += displacement;
            }
        }
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.drop_speed = speed;
    }

    pub fn hit_ceiling(&mut self) -> bool {
        for tile in &self.tiles {
            if tile.y < 0 {
                return true;
            }
        }
        return false;
    }

    pub fn clear_rows(&mut self) {
        for y in 0..self.dimension.y + 1 {
            let row: Vec<Vec2> = (0..self.dimension.x + 1).map(|x| Vec2::xy(x, y)).collect();
            if row.iter().all(|item| self.tiles.contains(item)) {
                &self.tiles.retain(|t| t.y != y);
                for tile in &mut self.tiles {
                    if tile.y < y {
                        tile.y += 1;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut app = App::config(Config { fps: 60 });
    let mut state = GameState::new(Vec2::xy(9, 19));

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::R) => state.rotate_piece(),
                KeyEvent::Released(Key::Space) => state.set_speed(20),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::A | Key::H => state.tile_move_x(-1, app_state.step()),
                Key::D | Key::L => state.tile_move_x(1, app_state.step()),
                Key::Space => state.set_speed(0),
                _ => (),
            }
        }

        state.update(app_state.step());

        let win_size = window.size();
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.set_origin((win_size - state.dimension) / 2);

        if state.hit_ceiling() {
            let status_msg = "You lose :(";
            let msg = &format!("{}", status_msg);
            pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
            pencil.draw_text(msg, Vec2::zero());
            return ();
        }

        state.clear_rows();

        pencil.set_foreground(Color::Cyan);
        for pos in state.current_piece.relative_tiles() {
            if pos.y >= 0 {
                pencil.draw_char('■', Vec2::xy(pos.x * 2, pos.y));
            }
        }
        pencil.set_foreground(Color::Green);
        for tile in &state.tiles {
            pencil.draw_char('■', Vec2::xy(tile.x * 2, tile.y));
        }

        pencil.set_foreground(Color::Red);
        for y in 0..state.dimension.y + 3 {
            for x in 0..state.dimension.x + 3 {
                if y == 0 || x == 0 || y == state.dimension.y + 2 || x == state.dimension.x + 2 {
                    pencil.draw_char('■', Vec2::xy(x * 2 - 2, y - 1));
                }
            }
        }
    });
}
