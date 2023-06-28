use rand::{ distributions::{Distribution, Standard}, Rng };
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use ruscii::{ app::{App, Config, State}, terminal::VisualElement };

enum TetriminoOffsets {
    O([Vec2; 4]),
    Other([[Vec2; 4]; 5]),
}

struct TetriminoDefinition {
    color: Color,
    tiles: [Vec2; 4],
    offsets: TetriminoOffsets,
}

enum TetriminoType {
    J,
    L,
    S,
    T,
    Z,
    I,
    O,
}

const I_OFFSETS_TABLE: TetriminoOffsets = TetriminoOffsets::Other([
    [Vec2 {x: 0, y: 0}, Vec2 {x: -1, y: 0}, Vec2 {x: -1, y: 1}, Vec2 {x: 0, y: 1}],
    [Vec2 {x: -1, y: 0}, Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 1}, Vec2 {x: 0, y: 1}],
    [Vec2 {x: 2, y: 0}, Vec2 {x: 0, y: 0}, Vec2 {x: -2, y: 1}, Vec2 {x: 0, y: 1}],
    [Vec2 {x: -1, y: 0}, Vec2 {x: 0, y: 1}, Vec2 {x: 1, y: 0}, Vec2 {x: 0, y: -1}],
    [Vec2 {x: 2, y: 0}, Vec2 {x: 0, y: -2}, Vec2 {x: -2, y: 0}, Vec2 {x: 0, y: 2}],
]);

const O_OFFSETS: TetriminoOffsets = TetriminoOffsets::O(
    [Vec2 {x: 0, y: 0}, Vec2 {x: 0, y: -1}, Vec2 {x: -1, y: -1}, Vec2 {x: -1, y: 0}]
);

const OTHER_OFFSETS_TABLE: TetriminoOffsets = TetriminoOffsets::Other([
    [Vec2 {x: 0, y: 0}; 4],
    [Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 0}, Vec2 {x: 0, y: 0}, Vec2 {x: -1, y: 0}],
    [Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: -1}, Vec2 {x: 0, y: 0}, Vec2 {x: -1, y: -1}],
    [Vec2 {x: 0, y: 0}, Vec2 {x: 0, y: 2}, Vec2 {x: 0, y: 0}, Vec2 {x: 0, y: 2}],
    [Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 2}, Vec2 {x: 0, y: 0}, Vec2 {x: -1, y: 2}],
]);

const J: TetriminoDefinition = TetriminoDefinition {
    color: Color::Xterm(9),
    tiles: [Vec2 {x: -1, y: -1}, Vec2 {x: -1, y: 0}, Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 0}],
    offsets: OTHER_OFFSETS_TABLE,
};

const L: TetriminoDefinition = TetriminoDefinition {
    color: Color::Xterm(10),
    tiles: [Vec2 {x: 1, y: -1}, Vec2 {x: -1, y: 0}, Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 0}],
    offsets: OTHER_OFFSETS_TABLE,
};

const S: TetriminoDefinition = TetriminoDefinition {
    color: Color::Xterm(11),
    tiles: [Vec2 {x: 1, y: -1}, Vec2 {x: 0, y: -1}, Vec2 {x: 0, y: 0}, Vec2 {x: -1, y: 0}],
    offsets: OTHER_OFFSETS_TABLE,
};

const T: TetriminoDefinition = TetriminoDefinition {
    color: Color::Xterm(12),
    tiles: [Vec2 {x: -1, y: 0}, Vec2 {x: 0, y: -1}, Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 0}],
    offsets: OTHER_OFFSETS_TABLE,
};

const Z: TetriminoDefinition = TetriminoDefinition {
    color: Color::Xterm(13),
    tiles: [Vec2 {x: 1, y: 0}, Vec2 {x: 0, y: -1}, Vec2 {x: 0, y: 0}, Vec2 {x: -1, y: -1}],
    offsets: OTHER_OFFSETS_TABLE,
};

const I: TetriminoDefinition = TetriminoDefinition {
    color: Color::Xterm(14),
    tiles: [Vec2 {x: -1, y: 0}, Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 0}, Vec2 {x: 2, y: 0}],
    offsets: I_OFFSETS_TABLE,
};

const O: TetriminoDefinition = TetriminoDefinition {
    color: Color::Xterm(15),
    tiles: [Vec2 {x: 1, y: -1}, Vec2 {x: 0, y: -1}, Vec2 {x: 0, y: 0}, Vec2 {x: 1, y: 0}],
    offsets: O_OFFSETS,
};

impl Distribution<TetriminoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetriminoType {
        match rng.gen_range(0..7) {
            0 => TetriminoType::J,
            1 => TetriminoType::L,
            2 => TetriminoType::S,
            3 => TetriminoType::T,
            4 => TetriminoType::Z,
            5 => TetriminoType::I,
            _ => TetriminoType::O,
        }
    }
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
struct Tetrimino<'a> {
    definition: &'a TetriminoDefinition,
    tiles: [Vec2; 4],
    location: Vec2,
    rotation_index: usize,
}

impl Tetrimino<'_> {
    pub fn new(location: Vec2) -> Self {
        let definition = match rand::random::<TetriminoType>() {
            TetriminoType::J => &J,
            TetriminoType::L => &L,
            TetriminoType::S => &S,
            TetriminoType::T => &T,
            TetriminoType::Z => &Z,
            TetriminoType::I => &I,
            TetriminoType::O => &O,
        };
        let mut tiles = definition.tiles.clone();
        tiles.iter_mut().for_each(|t| *t += location);

        Self {
            definition,
            tiles,
            location,
            rotation_index: 0,
        }
    }

    pub fn move_self(&mut self, direction: Vec2) {
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
        for tile in &self.tiles {
            match Tile::new(*tile + movement) {
                t if blocking_tiles.contains(&t) => { return false; },
                t if t.x < 0 => { return false; },
                t if t.x > arena_dimensions.x => { return false; },
                t if t.y > arena_dimensions.y => { return false; },
                _ => {()},
            }
        }
        true
    }

    fn offset(
        &mut self,
        new_rotation_index: usize,
        blocking_tiles: &[Tile],
        arena_dimensions: &Vec2,
    ) -> bool {
        let mut offset: Vec2;
        match self.definition.offsets {
            TetriminoOffsets::O(offsets) => {
                offset = offsets[self.rotation_index] - offsets[new_rotation_index];
                self.move_self(offset);
                true
            }

            TetriminoOffsets::Other(offsets_table) => {
                for offsets in offsets_table.iter() {
                    offset = offsets[self.rotation_index] - offsets[new_rotation_index];

                    if !self.can_move(offset, blocking_tiles, arena_dimensions) {
                        continue;
                    }

                    self.move_self(offset);
                    return true;
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

        let new_rotation_index = ((self.rotation_index as i32 + direction) % 4) as usize;

        let tilemap = self.get_tilemap();
        let new_tilemap: [Vec2; 4] = core::array::from_fn(
            |i| Vec2::xy(
                (rotation_matrix[0].x * tilemap[i].x)
                    + (rotation_matrix[1].x * tilemap[i].y),
                (rotation_matrix[0].y * tilemap[i].x)
                    + (rotation_matrix[1].y * tilemap[i].y),
            )
        );

        self.set_tiles(new_tilemap);

        if !do_offset {
            return;
        }

        match self.offset(new_rotation_index, blocking_tiles, arena_dimensions) {
            true => { self.rotation_index = new_rotation_index; }
            false => self.rotate(!clockwise, blocking_tiles, arena_dimensions, false)
        }
    }
}

struct GameState<'a> {
    pub dimension: Vec2,
    pub tiles: Vec<Tile>,
    pub current_tetrimino: Tetrimino<'a>,
    pub score: u32,
    pub last_update: usize,
    pub last_input: (usize, i32), // frame, direction
    pub drop_speed: usize,
    pub hit_ceiling: bool,
}

impl GameState<'_> {
    pub fn new(dimension: Vec2) -> GameState<'static> {
        GameState {
            dimension,
            tiles: Vec::with_capacity(dimension.x as usize * dimension.y as usize),
            last_update: 0,
            last_input: (0, 0),
            drop_speed: 20,
            score: 0,
            current_tetrimino: Tetrimino::new(Vec2::xy(dimension.x / 2, 0)),
            hit_ceiling: false,
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.last_update + self.drop_speed >= frame {
            return;
        }
            
        self.last_update = frame;
        
        if self.current_tetrimino.can_move(Vec2::xy(0, 1), &self.tiles, &self.dimension) {
            self.current_tetrimino.move_self(Vec2::xy(0, 1));
            return;
        }
                
        // make piece part of current tile set and spawn a new piece
        for tile in &self.current_tetrimino.tiles {
            self.tiles.push(
                Tile::with_color(*tile, self.current_tetrimino.definition.color));
            
            if tile.y >= 0 {
                continue;
            }

            self.hit_ceiling = true;
        }

        self.spawn_tetrimino();
    }

    pub fn rotate_tetrimino(&mut self, clockwise: bool) {
        self.current_tetrimino.rotate(clockwise, &self.tiles, &self.dimension, true);
    }

    pub fn spawn_tetrimino(&mut self) {
        self.current_tetrimino = Tetrimino::new(Vec2::xy(self.dimension.x / 2, 0))
    }

    pub fn tile_move_x(&mut self, displacement: i32, frame: usize) {
        if self.last_input.0 + 5 >= frame && self.last_input.1 == displacement {
            return;
        }

        self.last_input = (frame, displacement);
        let movement = Vec2::xy(displacement, 0);

        if !self.current_tetrimino.can_move(movement, &self.tiles, &self.dimension) {
            return;
        }

        self.current_tetrimino.move_self(movement);
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.drop_speed = speed;
    }

    pub fn clear_rows(&mut self) {
        for y in 0..self.dimension.y + 1 {
            let row: Vec<Vec2> = (0..self.dimension.x + 1)
                .map(|x| Vec2::xy(x, y))
                .collect();

            if !row.iter().all(|item| self.tiles.contains(&Tile::new(*item))) {
                continue;
            }

            self.score += 1;
            self.tiles.retain(|t| t.y != y);

            for tile in &mut self.tiles {
                if tile.y >= y {
                    continue;
                }

                tile.y += 1;
            }
        }
    }
}

fn main() {
    let controls_text: [&str; 5] = [
        "⇦ ⇨ : move left/right",
        "X, ⇧ : rotate clockwise",
        "Z : rotate counterclockwise",
        "SPACE, ⇩ : drop",
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
                KeyEvent::Pressed(Key::Z) => state.rotate_tetrimino(false),
                KeyEvent::Pressed(Key::X) | KeyEvent::Pressed(Key::Up) => {
                    state.rotate_tetrimino(true);
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

        if state.hit_ceiling {
            let msg = format!("You lose :( score: {}", state.score);
            pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
            pencil.draw_text(&msg, Vec2::zero());
            return;
        }

        state.clear_rows();
        pencil.set_foreground(state.current_tetrimino.definition.color);

        for pos in &state.current_tetrimino.tiles {
            if pos.y < 0 {
                continue;
            }

            pencil.draw_char('[', Vec2::xy(pos.x * 2, pos.y));
            pencil.draw_char(']', Vec2::xy(pos.x * 2 + 1, pos.y));
        }

        for tile in &state.tiles {
            pencil.set_foreground(tile.color);
            pencil.draw_char('[', Vec2::xy(tile.x * 2, tile.y));
            pencil.draw_char(']', Vec2::xy(tile.x * 2 + 1, tile.y));
        }

        pencil.set_foreground(Color::Xterm(8));

        for y in 0..=state.dimension.y + 1 {
            pencil.draw_char('│', Vec2::xy(-1, y));
            pencil.draw_char('│', Vec2::xy(2 * state.dimension.x + 2, y));
        }

        for x in 0..=2 * state.dimension.x + 1 {
            pencil.draw_char('─', Vec2::xy(x, 0));
            pencil.draw_char('─', Vec2::xy(x, state.dimension.y + 1));
        }

        pencil.set_foreground(Color::Xterm(15));
        pencil.set_origin(Vec2::xy(origin.x + state.dimension.x * 2 + 6, origin.y));
        pencil.draw_text(&format!("score: {}", state.score), Vec2::xy(0, 0));

        for (i, ctrl) in controls_text.iter().enumerate() {
            pencil.draw_text(ctrl, Vec2::xy(0, 2 + i));
        }
    });
}
