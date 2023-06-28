use rand::{ distributions::{Distribution, Standard}, Rng };
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use ruscii::{ app::{App, Config, State}, terminal::VisualElement };

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

enum PieceOffset {
    O([Vec2; 4]),
    Other([[Vec2; 4]; 5]),
}

impl PieceType {
    pub fn get_color(&self) -> Color {
        match self {
            PieceType::J => Color::Xterm(9),
            PieceType::L => Color::Xterm(10),
            PieceType::S => Color::Xterm(11),
            PieceType::T => Color::Xterm(12),
            PieceType::Z => Color::Xterm(13),
            PieceType::I => Color::Xterm(14),
            PieceType::O => Color::Xterm(15),
        }
    }
    pub fn get_tiles(&self) -> [Vec2; 4] {
        match self {
            PieceType::J => {
                [Vec2::xy(-1, -1), Vec2::xy(-1, 0), Vec2::xy(0, 0), Vec2::xy(1, 0)]
            }
            PieceType::L => {
                [Vec2::xy(1, -1), Vec2::xy(-1, 0), Vec2::xy(0, 0), Vec2::xy(1, 0)]
            }
            PieceType::S => {
                [Vec2::xy(1, -1), Vec2::xy(0, -1), Vec2::xy(0, 0), Vec2::xy(-1, 0)]
            }
            PieceType::T => {
                [Vec2::xy(-1, 0), Vec2::xy(0, -1), Vec2::xy(0, 0), Vec2::xy(1, 0)]
            }
            PieceType::Z => {
                [Vec2::xy(1, 0), Vec2::xy(0, -1), Vec2::xy(0, 0), Vec2::xy(-1, -1)]
            }
            PieceType::I => {
                [Vec2::xy(-1, 0), Vec2::xy(0, 0), Vec2::xy(1, 0), Vec2::xy(2, 0)]
            }
            PieceType::O => {
                [Vec2::xy(1, -1), Vec2::xy(0, -1), Vec2::xy(0, 0), Vec2::xy(1, 0)]
            }
        }
    }
    pub fn offset_data(&self) -> PieceOffset {
        match self {
            PieceType::I => {
                PieceOffset::Other([
                    [Vec2::zero(), Vec2::xy(-1, 0), Vec2::xy(-1, 1), Vec2::xy(0, 1)],
                    [Vec2::xy(-1, 0), Vec2::zero(), Vec2::xy(1, 1), Vec2::xy(0, 1)],
                    [Vec2::xy(2, 0), Vec2::zero(), Vec2::xy(-2, 1), Vec2::xy(0, 1)],
                    [Vec2::xy(-1, 0), Vec2::xy(0, 1), Vec2::xy(1, 0), Vec2::xy(0, -1)],
                    [Vec2::xy(2, 0), Vec2::xy(0, -2), Vec2::xy(-2, 0), Vec2::xy(0, 2)],
                ])
            }

            PieceType::O => {
                PieceOffset::O(
                    [Vec2::zero(), Vec2::xy(0, -1), Vec2::xy(-1, -1), Vec2::xy(-1, 0)]
                )
            }

            _ => {
                PieceOffset::Other([
                    [Vec2::zero(); 4],
                    [Vec2::zero(), Vec2::xy(1, 0), Vec2::zero(), Vec2::xy(-1, 0)],
                    [Vec2::zero(), Vec2::xy(1, -1), Vec2::zero(), Vec2::xy(-1, -1)],
                    [Vec2::zero(), Vec2::xy(0, 2), Vec2::zero(), Vec2::xy(0, 2)],
                    [Vec2::zero(), Vec2::xy(1, 2), Vec2::zero(), Vec2::xy(-1, 2)],
                ])
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

#[derive(Clone, Eq)]
struct Tile {
    color: Color,
    location: Vec2,
}

impl std::ops::Deref for Tile {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.location
    }
}

impl std::ops::DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.location
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

impl Tile {
    pub fn new(location: Vec2) -> Self {
        Self {
            color: Color::Xterm(8),
            location,
        }
    }

    pub fn with_color(location: Vec2, color: Color) -> Self {
        Self { color, location }
    }
}

#[derive(Clone)]
struct Piece {
    piece_type: PieceType,
    tiles: [Vec2; 4],
    location: Vec2,
    rotation_index: usize,
}

impl Piece {
    pub fn new(location: Vec2) -> Self {
        let piece_type: PieceType = rand::random();
        let mut tiles = piece_type.get_tiles().clone();
        tiles.iter_mut().for_each(|t| *t += location);

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

    fn set_tiles(&mut self, tilemap: [Vec2; 4]) {
        self.tiles
            .iter_mut()
            .enumerate()
            .for_each(|(i, t)| *t = tilemap[i] + self.location);
    }

    fn get_tilemap(&self) -> [Vec2; 4] {
        let mut tilemap: [Vec2; 4] = self.tiles.clone();
        tilemap.iter_mut().for_each(|t| *t -= self.location);
        tilemap
    }

    fn can_move(
        &self, movement: Vec2, blocking_tiles: &[Tile], arena_dimensions: &Vec2) -> bool {
        let mut can_move = true;

        for tile in &self.tiles {
            let future_tile = Tile::new(*tile + movement);
            if blocking_tiles.contains(&future_tile)
                || future_tile.x < 0
                || future_tile.x > arena_dimensions.x
                || future_tile.y > arena_dimensions.y
            {
                can_move = false;
                break;
            }
        }

        can_move
    }

    fn offset(
        &mut self,
        new_rotation_index: usize,
        blocking_tiles: &[Tile],
        arena_dimensions: &Vec2,
    ) -> bool {
        let mut offset: Vec2;
        match self.piece_type.offset_data() {
            PieceOffset::O(offsets) => {
                offset = offsets[self.rotation_index] - offsets[new_rotation_index];
                self.move_piece(offset);
                true
            }

            PieceOffset::Other(offsets_table) => {
                for offsets in offsets_table.iter() {
                    offset = offsets[self.rotation_index] - offsets[new_rotation_index];
                    if self.can_move(offset, blocking_tiles, arena_dimensions) {
                        self.move_piece(offset);
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn rotate(
        &mut self,
        clockwise: bool,
        blocking_tiles: &Vec<Tile>,
        arena_dimensions: &Vec2,
        do_offset: bool,
    ) {
        let direction = match clockwise {
            true => 1,
            false => -1,
        };

        let rotation_matrix = match clockwise {
            true => [Vec2::xy(0, 1), Vec2::xy(-1, 0)],
            false => [Vec2::xy(0, -1), Vec2::xy(1, 0)],
        };

        let new_rotation_index = modulo(
            self.rotation_index as i32 + direction, 4) as usize;

        let mut tilemap = [Vec2::zero(); 4];
        for (i, tile) in self.get_tilemap().iter().enumerate() {
            tilemap[i].x = (rotation_matrix[0].x * tile.x)
                + (rotation_matrix[1].x * tile.y);
            tilemap[i].y = (rotation_matrix[0].y * tile.x)
                + (rotation_matrix[1].y * tile.y);
        }

        self.set_tiles(tilemap);
        if !do_offset {
            return;
        }

        let can_rotate = self.offset(
            new_rotation_index, blocking_tiles, arena_dimensions);
        if !can_rotate {
            self.rotate(!clockwise, blocking_tiles, arena_dimensions, false)
        } else {
            self.rotation_index = new_rotation_index;
        }
    }
}

struct GameState {
    pub dimension: Vec2,
    pub tiles: Vec<Tile>,
    pub current_piece: Piece,
    pub score: u32,
    pub last_update: usize,
    pub last_input: (usize, i32), // frame, direction
    pub drop_speed: usize,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        GameState {
            dimension,
            tiles: Vec::with_capacity(dimension.x as usize * dimension.y as usize),
            last_update: 0,
            last_input: (0, 0),
            drop_speed: 20,
            score: 0,
            current_piece: Piece::new(Vec2::xy(dimension.x / 2, 0)),
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.last_update + self.drop_speed < frame {
            self.last_update = frame;
            if self.current_piece.can_move(Vec2::xy(0, 1), &self.tiles, &self.dimension)
            {
                self.current_piece.move_piece(Vec2::xy(0, 1));
            } else {
                // make piece part of current tile set and spawn a new piece
                for tile in &self.current_piece.tiles {
                    self.tiles.push(Tile::with_color(
                        *tile,
                        self.current_piece.piece_type.get_color(),
                    ))
                }
                self.spawn_piece();
            }
        }
    }

    pub fn rotate_piece(&mut self, clockwise: bool) {
        self.current_piece.rotate(clockwise, &self.tiles, &self.dimension, true);
    }

    pub fn spawn_piece(&mut self) {
        self.current_piece = Piece::new(Vec2::xy(self.dimension.x / 2, 0))
    }

    pub fn tile_move_x(&mut self, displacement: i32, frame: usize) {
        if self.last_input.0 + 5 < frame || self.last_input.1 != displacement {
            self.last_input = (frame, displacement);
            let movement = Vec2::xy(displacement, 0);

            if self.current_piece.can_move(movement, &self.tiles, &self.dimension)
            {
                self.current_piece.move_piece(movement);
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
        false
    }

    pub fn clear_rows(&mut self) {
        for y in 0..self.dimension.y + 1 {
            let row: Vec<Vec2> = (0..self.dimension.x + 1)
                .map(|x| Vec2::xy(x, y))
                .collect();

            if row.iter().all(|item| self.tiles.contains(&Tile::new(*item)))
            {
                self.score += 1;
                self.tiles.retain(|t| t.y != y);
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
    let controls_text: [&str; 5] = [
        "⇦ ⇨ : move piece",
        "X, ⇧ : rotate clockwise",
        "Z : rotate counterclockwise",
        "SPACE, ⇩ : drop piece",
        "ESC, Q : quit game",
    ];

    let mut app = App::config(Config { fps: 60 });
    let mut state = GameState::new(Vec2::xy(9, 19));
    let mut default = VisualElement::new();

    // use xterm 0 for background color
    default.background = Color::Xterm(0);

    app.run(|app_state: &mut State, window: &mut Window| {
        window.canvas_mut().set_default_element(&default);

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) | KeyEvent::Pressed(Key::Q) => {
                    app_state.stop();
                }
                KeyEvent::Pressed(Key::Z) => state.rotate_piece(false),
                KeyEvent::Pressed(Key::X) | KeyEvent::Pressed(Key::Up) => {
                    state.rotate_piece(true);
                }
                KeyEvent::Released(Key::Left) => {
                    state.last_input = (0, -1);
                }
                KeyEvent::Released(Key::Right) => {
                    state.last_input = (0, 1);
                }
                KeyEvent::Released(Key::Space) | KeyEvent::Released(Key::Down) => {
                    state.set_speed(20)
                }
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::Left => state.tile_move_x(-1, app_state.step()),
                Key::Right => state.tile_move_x(1, app_state.step()),
                Key::Space | Key::Down => state.set_speed(0),
                _ => (),
            }
        }

        state.update(app_state.step());
        let win_size = window.size();
        let mut pencil = Pencil::new(window.canvas_mut());
        let origin = (win_size - Vec2::xy(state.dimension.x * 2, state.dimension.y)) / 2;
        pencil.set_origin(origin);

        if state.hit_ceiling() {
            let msg = format!("You lose :( score: {}", state.score);
            pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
            pencil.draw_text(&msg, Vec2::zero());
            return;
        }

        state.clear_rows();
        pencil.set_foreground(state.current_piece.piece_type.get_color());

        for pos in &state.current_piece.tiles {
            if pos.y >= 0 {
                pencil.draw_char('[', Vec2::xy(pos.x * 2, pos.y));
                pencil.draw_char(']', Vec2::xy(pos.x * 2 + 1, pos.y));
            }
        }

        for tile in &state.tiles {
            pencil.set_foreground(tile.color);
            pencil.draw_char('[', Vec2::xy(tile.x * 2, tile.y));
            pencil.draw_char(']', Vec2::xy(tile.x * 2 + 1, tile.y));
        }

        pencil.set_foreground(Color::Xterm(8));

        for y in 0..state.dimension.y + 3 {
            for x in 0..state.dimension.x + 3 {
                if x == 0 {
                    pencil.draw_char('│', Vec2::xy(x * 2 - 1, y - 1));
                } else if x == state.dimension.x + 2 {
                    pencil.draw_char('│', Vec2::xy(x * 2 - 2, y - 1));
                } else if y == 0 || y == state.dimension.y + 2 {
                    pencil.draw_char('─', Vec2::xy(x * 2 - 2, y - 1));
                    pencil.draw_char('─', Vec2::xy(x * 2 - 1, y - 1));
                }
            }
        }

        pencil.set_foreground(Color::Xterm(15));
        pencil.set_origin(Vec2::xy(origin.x + state.dimension.x * 2 + 6, origin.y));
        pencil.draw_text(&format!("score: {}", state.score), Vec2::xy(0, 0));

        for (i, ctrl) in controls_text.iter().enumerate() {
            pencil.draw_text(ctrl, Vec2::xy(0, 2 + i));
        }
    });
}
