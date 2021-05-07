use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ruscii::app::{App, Config, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

#[derive(Clone)]
enum PieceType {
    J,
    L,
    S,
    T,
    Z,
    I,
    O,
}

impl PieceType {
    pub fn get_color(&self) -> Color {
        match self {
            PieceType::J => Color::Blue,
            PieceType::L => Color::Grey,
            PieceType::S => Color::Green,
            PieceType::T => Color::Magenta,
            PieceType::Z => Color::Red,
            PieceType::I => Color::Cyan,
            PieceType::O => Color::Yellow,
        }
    }
    pub fn get_tiles(&self) -> Vec<Vec2> {
        match self {
            PieceType::J => {
                vec![
                    Vec2::xy(-1, -1),
                    Vec2::xy(-1, 0),
                    Vec2::xy(0, 0),
                    Vec2::xy(1, 0),
                ]
            }
            PieceType::L => {
                vec![
                    Vec2::xy(1, -1),
                    Vec2::xy(-1, 0),
                    Vec2::xy(0, 0),
                    Vec2::xy(1, 0),
                ]
            }
            PieceType::S => {
                vec![
                    Vec2::xy(1, -1),
                    Vec2::xy(0, -1),
                    Vec2::xy(0, 0),
                    Vec2::xy(-1, 0),
                ]
            }
            PieceType::T => {
                vec![
                    Vec2::xy(-1, 0),
                    Vec2::xy(0, -1),
                    Vec2::xy(0, 0),
                    Vec2::xy(1, 0),
                ]
            }
            PieceType::Z => {
                vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(0, -1),
                    Vec2::xy(0, 0),
                    Vec2::xy(-1, -1),
                ]
            }
            PieceType::I => {
                vec![
                    Vec2::xy(1, -1),
                    Vec2::xy(0, -1),
                    Vec2::xy(0, 0),
                    Vec2::xy(-1, 0),
                ]
            }
            PieceType::O => {
                vec![
                    Vec2::xy(-1, 0),
                    Vec2::xy(0, 0),
                    Vec2::xy(1, 0),
                    Vec2::xy(2, 0),
                ]
            }
        }
    }
    pub fn offset_data(&self) -> Vec<Vec<Vec2>> {
        match self {
            PieceType::I => {
                vec![
                    vec![
                        Vec2::zero(),
                        Vec2::xy(-1, 0),
                        Vec2::xy(-1, 1),
                        Vec2::xy(0, 1),
                    ],
                    vec![
                        Vec2::xy(-1, 0),
                        Vec2::zero(),
                        Vec2::xy(1, 1),
                        Vec2::xy(0, 1),
                    ],
                    vec![
                        Vec2::xy(2, 0),
                        Vec2::zero(),
                        Vec2::xy(-2, 1),
                        Vec2::xy(0, 1),
                    ],
                    vec![
                        Vec2::xy(-1, 0),
                        Vec2::xy(0, 1),
                        Vec2::xy(1, 0),
                        Vec2::xy(0, -1),
                    ],
                    vec![
                        Vec2::xy(2, 0),
                        Vec2::xy(0, -2),
                        Vec2::xy(-2, 0),
                        Vec2::xy(0, 2),
                    ],
                ]
            }
            PieceType::O => {
                vec![vec![
                    Vec2::zero(),
                    Vec2::xy(0, -1),
                    Vec2::xy(-1, -1),
                    Vec2::xy(-1, 0),
                ]]
            }
            _ => {
                vec![
                    vec![Vec2::zero(); 4],
                    vec![Vec2::zero(), Vec2::xy(1, 0), Vec2::zero(), Vec2::xy(-1, 0)],
                    vec![
                        Vec2::zero(),
                        Vec2::xy(1, -1),
                        Vec2::zero(),
                        Vec2::xy(-1, -1),
                    ],
                    vec![Vec2::zero(), Vec2::xy(0, 2), Vec2::zero(), Vec2::xy(0, 2)],
                    vec![Vec2::zero(), Vec2::xy(1, 2), Vec2::zero(), Vec2::xy(-1, 2)],
                ]
            }
        }
    }
}

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0..7) {
            0 => PieceType::J,
            1 => PieceType::L,
            2 => PieceType::S,
            3 => PieceType::T,
            4 => PieceType::Z,
            5 => PieceType::I,
            _ => PieceType::O,
        }
    }
}

fn modulo(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}

#[derive(Clone)]
struct Piece {
    piece_type: PieceType,
    tiles: Vec<Vec2>,
    location: Vec2,
    rotation_index: usize,
}

impl Piece {
    pub fn new(location: Vec2) -> Self {
        let piece_type: PieceType = rand::random();
        let tiles = piece_type
            .get_tiles()
            .iter()
            .map(|&t| t + location)
            .collect();
        Self {
            piece_type,
            tiles,
            location,
            rotation_index: 0,
        }
    }

    pub fn move_piece(&mut self, direction: Vec2) {
        self.location += direction;
        for tile in &mut self.tiles {
            *tile += direction;
        }
    }

    fn set_tiles(&mut self, tilemap: Vec<Vec2>) {
        let tiles: Vec<Vec2> = tilemap.iter().map(|&t| t + self.location).collect();
        self.tiles = tiles;
    }

    fn get_tilemap(&self) -> Vec<Vec2> {
        let mut tilemap: Vec<Vec2> = Vec::new();
        for tile in &self.tiles {
            tilemap.push(*tile - self.location);
        }
        tilemap
    }

    fn can_move(
        &self,
        movement: Vec2,
        blocking_tiles: &Vec<Vec2>,
        arena_dimensions: &Vec2,
    ) -> bool {
        let mut can_move = false;

        for tile in &self.tiles {
            let future_tile = tile.clone() + movement;
            if !blocking_tiles.contains(&future_tile)
                && future_tile.x >= 0
                && future_tile.x < arena_dimensions.x
            {
                can_move = true;
                break;
            }
        }

        can_move
    }

    fn offset(
        &mut self,
        new_rotation_index: usize,
        blocking_tiles: &Vec<Vec2>,
        arena_dimensions: &Vec2,
    ) -> bool {
        let mut offset_1: Vec2;
        let mut offset_2: Vec2;
        let mut end_offset: Vec2 = Vec2::zero();
        let mut move_possible: bool = false;

        let dataset = self.piece_type.offset_data();

        for test_index in 0..dataset.len() {
            offset_1 = dataset[test_index][self.rotation_index as usize];
            offset_2 = dataset[test_index][new_rotation_index as usize];
            end_offset = offset_1 - offset_2;
            if self.can_move(end_offset, blocking_tiles, arena_dimensions) {
                move_possible = true;
                break;
            }
        }

        if move_possible {
            self.move_piece(end_offset);
        }

        move_possible
    }

    pub fn rotate(
        &mut self,
        clockwise: bool,
        blocking_tiles: &Vec<Vec2>,
        arena_dimensions: &Vec2,
        do_offset: bool,
    ) {
        let mut tilemap = Vec::new();
        let direction = match clockwise {
            true => 1,
            false => -1,
        };
        let new_rotation_index = modulo(self.rotation_index as i32 + direction, 4) as usize;

        for tile in self.get_tilemap() {
            let rotation_matrix = match clockwise {
                true => vec![Vec2::xy(0, -1), Vec2::xy(1, 0)],
                false => vec![Vec2::xy(0, 1), Vec2::xy(-1, 0)],
            };
            let new_x = (rotation_matrix[0].x * tile.x) + (rotation_matrix[1].x * tile.y);
            let new_y = (rotation_matrix[0].y * tile.x) + (rotation_matrix[1].y * tile.y);
            tilemap.push(Vec2::xy(new_x, new_y));
        }

        self.set_tiles(tilemap);
        if !do_offset {
            return;
        }
        let can_rotate = self.offset(new_rotation_index, blocking_tiles, arena_dimensions);
        if !can_rotate {
            self.rotate(!clockwise, blocking_tiles, arena_dimensions, false)
        } else {
            self.rotation_index = new_rotation_index;
        }
    }
}

struct GameState {
    pub dimension: Vec2,
    pub tiles: Vec<Vec2>,
    pub current_piece: Piece,
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
            current_piece: Piece::new(Vec2::xy(dimension.x / 2, 0)),
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.last_update + self.drop_speed < frame {
            self.last_update = frame;
            if self.will_collide(Vec2::xy(0, 1), false) {
                for tile in &self.current_piece.tiles {
                    self.tiles.push(tile.clone())
                }
                self.spawn_piece();
            } else {
                self.current_piece.move_piece(Vec2::xy(0, 1));
            }
        }
    }

    pub fn rotate_piece(&mut self, clockwise: bool) {
        self.current_piece
            .rotate(clockwise, &self.tiles, &self.dimension, true);
    }

    pub fn spawn_piece(&mut self) {
        self.current_piece = Piece::new(Vec2::xy(self.dimension.x / 2, 0))
    }

    pub fn will_collide(&mut self, movement: Vec2, walls: bool) -> bool {
        let mut future_piece = self.current_piece.clone();
        future_piece.move_piece(movement);
        let mut collision = false;
        for piece_tile in future_piece.tiles {
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
        if self.last_input.0 + 5 < frame || self.last_input.1 != displacement {
            self.last_input = (frame, displacement);
            if !self.will_collide(Vec2::xy(displacement, 0), true) {
                self.current_piece.move_piece(Vec2::xy(displacement, 0));
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
                KeyEvent::Pressed(Key::Q) => state.rotate_piece(false),
                KeyEvent::Pressed(Key::R) => state.rotate_piece(true),
                KeyEvent::Released(Key::A) => {
                    state.last_input = (0, -1);
                }
                KeyEvent::Released(Key::D) => {
                    state.last_input = (0, 1);
                }
                KeyEvent::Released(Key::Space) | KeyEvent::Released(Key::S) => state.set_speed(20),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::A => state.tile_move_x(-1, app_state.step()),
                Key::D => state.tile_move_x(1, app_state.step()),
                Key::Space | Key::S => state.set_speed(0),
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

        pencil.set_foreground(state.current_piece.piece_type.get_color());
        for pos in &state.current_piece.tiles {
            if pos.y >= 0 {
                pencil.draw_char('■', Vec2::xy(pos.x * 2, pos.y));
            }
        }
        pencil.set_foreground(Color::DarkGrey);
        for tile in &state.tiles {
            pencil.draw_char('■', Vec2::xy(tile.x * 2, tile.y));
        }

        pencil.set_foreground(Color::White);
        for y in 0..state.dimension.y + 3 {
            for x in 0..state.dimension.x + 3 {
                if y == 0 || x == 0 || y == state.dimension.y + 2 || x == state.dimension.x + 2 {
                    pencil.draw_char('■', Vec2::xy(x * 2 - 2, y - 1));
                }
            }
        }
    });
}
