use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use ruscii::{
    app::{App, Config, State},
    terminal::VisualElement,
};

enum TetriminoWallKicks {
    O(Vec2),
    Other([Vec2; 5]),
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

enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

enum MovementCheck {
    CanMove,
    TouchingWall,
    TouchingStack,
}

#[derive(Clone)]
enum Rotation {
    Zero,
    Two,
    Left,
    Right,
}

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

struct TetriminoPrimitive {
    tetrimino_type: TetriminoType,
    color: Color,
    zero: [Vec2; 4],
    left: [Vec2; 4],
    right: [Vec2; 4],
    two: [Vec2; 4],
}

const fn rotate_right(input: [Vec2; 4]) -> [Vec2; 4] {
    [
        Vec2 {
            x: -input[0].y,
            y: input[0].x,
        },
        Vec2 {
            x: -input[1].y,
            y: input[1].x,
        },
        Vec2 {
            x: -input[2].y,
            y: input[2].x,
        },
        Vec2 {
            x: -input[3].y,
            y: input[3].x,
        },
    ]
}

impl TetriminoPrimitive {
    const fn new(tetrimino_type: TetriminoType, color: Color, zero: [Vec2; 4]) -> Self {
        let right = rotate_right(zero);
        let two = rotate_right(right);
        let left = rotate_right(two);

        Self {
            tetrimino_type,
            color,
            zero,
            left,
            right,
            two,
        }
    }

    pub fn get_random_primative() -> &'static Self {
        match rand::random::<TetriminoType>() {
            TetriminoType::J => &J,
            TetriminoType::L => &L,
            TetriminoType::S => &S,
            TetriminoType::T => &T,
            TetriminoType::Z => &Z,
            TetriminoType::I => &I,
            TetriminoType::O => &O,
        }
    }

    pub fn get_wall_kicks(&self, from: &Rotation, to: &Rotation) -> TetriminoWallKicks {
        let from_idx = match from {
            &Rotation::Zero => 0,
            &Rotation::Right => 1,
            &Rotation::Two => 2,
            &Rotation::Left => 3,
        };

        let to_idx = match to {
            &Rotation::Zero => 0,
            &Rotation::Right => 1,
            &Rotation::Two => 2,
            &Rotation::Left => 3,
        };

        match self.tetrimino_type {
            TetriminoType::O => TetriminoWallKicks::O(O_OFFSETS[from_idx] - O_OFFSETS[to_idx]),
            TetriminoType::I => TetriminoWallKicks::Other(core::array::from_fn(|i| {
                I_OFFSETS_TABLE[i][from_idx] - I_OFFSETS_TABLE[i][to_idx]
            })),
            _ => TetriminoWallKicks::Other(core::array::from_fn(|i| {
                OTHER_OFFSETS_TABLE[i][from_idx] - OTHER_OFFSETS_TABLE[i][to_idx]
            })),
        }
    }

    pub fn get_positions(&self, rotation: &Rotation) -> [Vec2; 4] {
        match rotation {
            &Rotation::Zero => self.zero,
            &Rotation::Left => self.left,
            &Rotation::Right => self.right,
            &Rotation::Two => self.two,
        }
    }
}

// use xterm 0 for background color
const BACKGROUND_COLOR: Color = Color::Xterm(0);
const BORDER_COLOR: Color = Color::Xterm(8);
const SCORE_COLOR: Color = Color::Xterm(15);
const CONTROLS_COLOR: Color = Color::Xterm(15);
const FPS: u32 = 60;
const FIELD_SIZE: Vec2 = Vec2 { x: 9, y: 19 };
const DROP_SPEED: usize = 0;
const TAP_DROP_SPEED: usize = 20;

const CONTROLS_TEXT: [&str; 5] = [
    "⇦ ⇨ : move left/right",
    "X, ⇧ : rotate clockwise",
    "Z : rotate counterclockwise",
    "SPACE, ⇩ : drop",
    "ESC, Q : quit game",
];

const I_OFFSETS_TABLE: [[Vec2; 4]; 5] = [
    [
        Vec2 { x: 0, y: 0 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: -1, y: -1 },
        Vec2 { x: 0, y: -1 },
    ],
    [
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: -1 },
        Vec2 { x: 0, y: -1 },
    ],
    [
        Vec2 { x: 2, y: 0 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: -2, y: -1 },
        Vec2 { x: 0, y: -1 },
    ],
    [
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: 1 },
    ],
    [
        Vec2 { x: 2, y: 0 },
        Vec2 { x: 0, y: -2 },
        Vec2 { x: -2, y: 0 },
        Vec2 { x: 0, y: -2 },
    ],
];

const O_OFFSETS: [Vec2; 4] = [
    Vec2 { x: 0, y: 0 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 1 },
    Vec2 { x: -1, y: 0 },
];

const OTHER_OFFSETS_TABLE: [[Vec2; 4]; 5] = [
    [Vec2 { x: 0, y: 0 }; 4],
    [
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: -1, y: 0 },
    ],
    [
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 1 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: -1, y: 1 },
    ],
    [
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 0, y: -2 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 0, y: -2 },
    ],
    [
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: -2 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: -1, y: -2 },
    ],
];

const J: TetriminoPrimitive = TetriminoPrimitive::new(
    TetriminoType::J,
    Color::Xterm(9),
    [
        Vec2 { x: -1, y: -1 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 0 },
    ],
);

const L: TetriminoPrimitive = TetriminoPrimitive::new(
    TetriminoType::L,
    Color::Xterm(10),
    [
        Vec2 { x: 1, y: -1 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 0 },
    ],
);

const S: TetriminoPrimitive = TetriminoPrimitive::new(
    TetriminoType::S,
    Color::Xterm(11),
    [
        Vec2 { x: 1, y: -1 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: -1, y: 0 },
    ],
);

const T: TetriminoPrimitive = TetriminoPrimitive::new(
    TetriminoType::T,
    Color::Xterm(12),
    [
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 0 },
    ],
);

const Z: TetriminoPrimitive = TetriminoPrimitive::new(
    TetriminoType::Z,
    Color::Xterm(13),
    [
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: -1, y: -1 },
    ],
);

const I: TetriminoPrimitive = TetriminoPrimitive::new(
    TetriminoType::I,
    Color::Xterm(14),
    [
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 2, y: 0 },
    ],
);

const O: TetriminoPrimitive = TetriminoPrimitive::new(
    TetriminoType::O,
    Color::Xterm(15),
    [
        Vec2 { x: 1, y: -1 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 0 },
    ],
);

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
}

#[derive(Clone)]
struct Tetrimino<'a> {
    primitive: &'a TetriminoPrimitive,
    origin: Vec2,
    rotation: Rotation,
}

impl Tetrimino<'_> {
    pub fn new(origin: Vec2) -> Self {
        let primitive = TetriminoPrimitive::get_random_primative();

        Self {
            primitive,
            origin,
            rotation: Rotation::Zero,
        }
    }

    pub fn translate(
        &mut self,
        translation: Vec2,
        blocking_tiles: &[Tile],
        arena_dimensions: &Vec2,
    ) -> Result<()> {
        let new_locations = self.get_locations(self.origin + translation, &self.rotation);
        match self.can_move(new_locations, blocking_tiles, arena_dimensions) {
            MovementCheck::CanMove => {
                self.origin += translation;
                true
            }
            MovementCheck::TouchingWall => true,
            MovementCheck::TouchingStack => false,
        }
    }

    fn get_locations(&self, origin: Vec2, rotation: &Rotation) -> [Vec2; 4] {
        let mut relative_locations = self.primitive.get_positions(&rotation);
        relative_locations.iter_mut().for_each(|p| *p += origin);
        relative_locations
    }

    fn get_tiles(&self) -> [Tile; 4] {
        let relative_positions = self.primitive.get_positions(&self.rotation);
        core::array::from_fn(|i| Tile {
            color: self.primitive.color,
            location: relative_positions[i] + self.origin,
        })
    }

    fn can_move(
        &self,
        new_locations: [Vec2; 4],
        blocking_tiles: &[Tile],
        arena_dimensions: &Vec2,
    ) -> MovementCheck {
        for location in new_locations {
            match Tile::new(location) {
                t if blocking_tiles.contains(&t) => {
                    return MovementCheck::TouchingStack;
                }
                t if t.y > arena_dimensions.y => {
                    return MovementCheck::TouchingStack;
                }
                t if t.x < 0 => {
                    return MovementCheck::TouchingWall;
                }
                t if t.x > arena_dimensions.x => {
                    return MovementCheck::TouchingWall;
                }
                _ => (),
            };
        }
        MovementCheck::CanMove
    }

    pub fn rotate(
        &mut self,
        rotation_direction: RotationDirection,
        blocking_tiles: &Vec<Tile>,
        arena_dimensions: &Vec2,
    ) {
        let new_rotation = match rotation_direction {
            RotationDirection::Clockwise => match self.rotation {
                Rotation::Zero => Rotation::Right,
                Rotation::Right => Rotation::Two,
                Rotation::Two => Rotation::Left,
                Rotation::Left => Rotation::Zero,
            },

            RotationDirection::Counterclockwise => match self.rotation {
                Rotation::Zero => Rotation::Left,
                Rotation::Left => Rotation::Two,
                Rotation::Two => Rotation::Right,
                Rotation::Right => Rotation::Zero,
            },
        };

        match self.primitive.get_wall_kicks(&self.rotation, &new_rotation) {
            TetriminoWallKicks::O(kick) => self.perform_rotation(kick, new_rotation),

            TetriminoWallKicks::Other(kicks) => {
                self.non_o_wall_kick_checks(kicks, new_rotation, blocking_tiles, arena_dimensions);
            }
        };
    }

    fn non_o_wall_kick_checks(
        &self,
        kicks: [Vec2; 5],
        new_rotation: Rotation,
        blocking_tiles: &[Tile],
        arena_dimensions: &Vec2,
    ) {
        let rotated_positions = self.primitive.get_positions(&new_rotation);
        for kick in kicks.iter() {
            let kicked_locations: [Vec2; 4] =
                core::array::from_fn(|i| *kick + rotated_positions[i] + self.origin);

            match self.can_move(kicked_locations, blocking_tiles, arena_dimensions) {
                MovementCheck::CanMove => {
                    self.perform_rotation(*kick, new_rotation);
                    break;
                }
                _ => (),
            }
        }
    }

    fn perform_rotation(&self, kick: Vec2, new_rotation: Rotation) {
        self.rotation = new_rotation;
        self.origin += kick;
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
            current_tetrimino: Tetrimino::new(Vec2::x(dimension.x / 2)),
            hit_ceiling: false,
        }
    }

    pub fn update(&mut self, frame: usize) {
        if self.last_update + self.drop_speed >= frame {
            return;
        }

        self.last_update = frame;

        if self
            .current_tetrimino
            .translate(Vec2::y(1), &self.tiles, &self.dimension)
        {
            return;
        }

        // make piece part of current tile set and spawn a new piece
        for tile in self.current_tetrimino.get_tiles() {
            self.tiles.push(tile.clone());

            if tile.y >= 0 {
                continue;
            }

            self.hit_ceiling = true;
        }

        self.spawn_tetrimino();
    }

    pub fn rotate_tetrimino(&mut self, rotation_direction: RotationDirection) {
        self.current_tetrimino
            .rotate(rotation_direction, &self.tiles, &self.dimension);
    }

    pub fn spawn_tetrimino(&mut self) {
        self.current_tetrimino = Tetrimino::new(Vec2::x(self.dimension.x / 2))
    }

    pub fn tile_move_x(&mut self, displacement: i32, frame: usize) {
        if self.last_input.0 + 5 >= frame && self.last_input.1 == displacement {
            return;
        }

        self.last_input = (frame, displacement);
        let movement = Vec2::x(displacement);

        self.current_tetrimino
            .translate(movement, &self.tiles, &self.dimension);
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.drop_speed = speed;
    }

    pub fn clear_rows(&mut self) {
        for y in 0..self.dimension.y + 1 {
            let row: Vec<Vec2> = (0..self.dimension.x + 1).map(|x| Vec2::xy(x, y)).collect();

            if !row
                .iter()
                .all(|item| self.tiles.contains(&Tile::new(*item)))
            {
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

fn draw_tile(pencil: &mut Pencil, tile: &Tile) {
    pencil.set_foreground(tile.color);
    pencil.draw_char('[', Vec2::xy(tile.x * 2, tile.y));
    pencil.draw_char(']', Vec2::xy(tile.x * 2 + 1, tile.y));
}

fn draw_border(pencil: &mut Pencil, state: &GameState) {
    pencil.set_foreground(BORDER_COLOR);

    for y in 0..=state.dimension.y + 1 {
        pencil.draw_char('│', Vec2::xy(-1, y));
        pencil.draw_char('│', Vec2::xy(2 * state.dimension.x + 2, y));
    }

    for x in 0..=2 * state.dimension.x + 1 {
        pencil.draw_char('─', Vec2::x(x));
        pencil.draw_char('─', Vec2::xy(x, state.dimension.y + 1));
    }
}

fn draw_score(pencil: &mut Pencil, game_state: &GameState) {
    pencil.set_foreground(SCORE_COLOR);
    pencil.draw_text(&format!("score: {}", game_state.score), Vec2::zero());
}

fn draw_controls(pencil: &mut Pencil) {
    pencil.set_foreground(CONTROLS_COLOR);
    for (i, ctrl) in CONTROLS_TEXT.iter().enumerate() {
        pencil.draw_text(ctrl, Vec2::y(2 + i));
    }
}

fn handle_controls(app_state: &State, game_state: &mut GameState) {
    for key_event in app_state.keyboard().last_key_events() {
        match key_event {
            KeyEvent::Pressed(Key::Esc) | KeyEvent::Pressed(Key::Q) => {
                app_state.stop();
            }
            KeyEvent::Pressed(Key::Z) => {
                game_state.rotate_tetrimino(RotationDirection::Counterclockwise);
            }
            KeyEvent::Pressed(Key::X) | KeyEvent::Pressed(Key::Up) => {
                game_state.rotate_tetrimino(RotationDirection::Clockwise);
            }
            KeyEvent::Released(Key::Left) => {
                game_state.last_input = (0, -1);
            }
            KeyEvent::Released(Key::Right) => {
                game_state.last_input = (0, 1);
            }
            KeyEvent::Released(Key::Space) | KeyEvent::Released(Key::Down) => {
                game_state.set_speed(TAP_DROP_SPEED)
            }
            _ => (),
        }
    }

    for key_down in app_state.keyboard().get_keys_down() {
        match key_down {
            Key::Left => game_state.tile_move_x(-1, app_state.step()),
            Key::Right => game_state.tile_move_x(1, app_state.step()),
            Key::Space | Key::Down => game_state.set_speed(DROP_SPEED),
            _ => (),
        }
    }
}

fn game_over(pencil: &mut Pencil, game_state: &GameState, window_size: Vec2) {
    let msg = format!("You lose :( score: {}", game_state.score);
    pencil.set_origin(window_size / 2 - Vec2::x(msg.len() / 2));
    pencil.draw_text(&msg, Vec2::zero());
}

fn game_loop(
    game_state: &mut GameState,
    default: VisualElement,
    app_state: &mut State,
    window: &mut Window,
) {
    let window_size = window.size();
    window.canvas_mut().set_default_element(&default);
    handle_controls(app_state, game_state);
    game_state.update(app_state.step());
    let mut pencil = Pencil::new(window.canvas_mut());
    let origin = (-Vec2::xy(2, 1) * game_state.dimension + window_size) / 2;
    pencil.set_origin(origin);

    if game_state.hit_ceiling {
        game_over(&mut pencil, &game_state, window_size);
        return;
    }

    game_state.clear_rows();

    for tile in game_state.current_tetrimino.get_tiles() {
        if tile.y < 0 {
            continue;
        }

        draw_tile(&mut pencil, &tile);
    }

    for tile in &game_state.tiles {
        draw_tile(&mut pencil, tile)
    }

    draw_border(&mut pencil, &game_state);
    pencil.set_origin(Vec2::x(game_state.dimension.x * 2 + 6) + origin);
    draw_score(&mut pencil, &game_state);
    draw_controls(&mut pencil);
}

fn main() {
    let mut app = App::config(Config { fps: FPS });
    let mut game_state = GameState::new(FIELD_SIZE);
    let mut default = VisualElement::new();
    default.background = BACKGROUND_COLOR;
    let runnable_game_loop = |app_state: &mut State, window: &mut Window| {
        game_loop(&mut game_state, default, app_state, window)
    };
    app.run(runnable_game_loop);
}
