use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Rect, Transform};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key::Named, NamedKey};
use winit::window::WindowBuilder;

// A size of pixel when drawing into a display
const UNIT_SIZE: i32 = 20;

// Titlis board size
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 22;

// Key definition
enum Key {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    SP,
    OTHER,
}

// Utility function that returns an index from (x, y) points
fn index_at(x: i32, y: i32) -> usize {
    (y * BOARD_WIDTH + x) as usize
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
    X,
}

impl Tetromino {
    fn rand() -> Self {
        match rand::random::<u32>() % 7 {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::J,
            4 => Tetromino::L,
            5 => Tetromino::S,
            6 => Tetromino::Z,
            _ => Tetromino::X,
        }
    }

    fn shape(&self) -> [[i32; 2]; 4] {
        match self {
            Tetromino::I => [[0, -1], [0, 0], [0, 1], [0, 2]],
            Tetromino::O => [[0, 0], [1, 0], [0, 1], [1, 1]],
            Tetromino::T => [[-1, 0], [0, 0], [1, 0], [0, -1]],
            Tetromino::J => [[-1, -1], [0, -1], [0, 0], [0, 1]],
            Tetromino::L => [[1, -1], [0, -1], [0, 0], [0, 1]],
            Tetromino::S => [[0, -1], [0, 0], [-1, 0], [-1, 1]],
            Tetromino::Z => [[0, -1], [0, 0], [1, 0], [1, 1]],
            Tetromino::X => [[0; 2]; 4],
        }
    }

    fn color(&self) -> (u8, u8, u8) {
        match self {
            Tetromino::I => (104, 102, 204),
            Tetromino::O => (204, 102, 204),
            Tetromino::T => (204, 204, 102),
            Tetromino::J => (204, 204, 204),
            Tetromino::L => (218, 170, 0),
            Tetromino::S => (204, 102, 102),
            Tetromino::Z => (102, 204, 102),
            _ => (0, 0, 0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Block {
    kind: Tetromino,
    points: [[i32; 2]; 4],
    x: i32,
    y: i32,
}

impl Block {
    fn new(x: i32, y: i32) -> Self {
        let kind = Tetromino::rand();
        Block {
            kind,
            points: kind.shape(),
            x,
            y: y - kind.shape().iter().max_by_key(|p| p[1]).unwrap()[1],
        }
    }

    fn empty() -> Self {
        let kind = Tetromino::X;
        Block {
            kind,
            points: kind.shape(),
            x: 0,
            y: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.kind == Tetromino::X
    }
    fn point(&self, i: usize) -> (i32, i32) {
        (self.x + self.points[i][0], self.y + self.points[i][1])
    }
    fn left(&self) -> Block {
        Block {
            x: self.x - 1,
            ..*self
        }
    }
    fn right(&self) -> Block {
        Block {
            x: self.x + 1,
            ..*self
        }
    }
    fn down(&self) -> Block {
        Block {
            y: self.y - 1,
            ..*self
        }
    }
    fn rotate_right(&self) -> Block {
        self.rotate(true)
    }
    fn rotate_left(&self) -> Block {
        self.rotate(false)
    }
    fn rotate(&self, clockwise: bool) -> Block {
        let mut points: [[i32; 2]; 4] = [[0; 2]; 4];
        for i in 0..4 {
            points[i] = if clockwise {
                // rotate_right
                [self.points[i][1], -self.points[i][0]]
            } else {
                // rotate_left
                [-self.points[i][1], self.points[i][0]]
            };
        }
        Block { points, ..*self }
    }
}

struct Game {
    board: [Tetromino; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
    current: Block,
    stopped: bool,
    time: std::time::SystemTime,
    score: u32,
}

impl Game {
    fn new() -> Self {
        Game {
            board: [Tetromino::X; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
            current: Block::empty(),
            stopped: false,
            time: std::time::SystemTime::now(),
            score: 0,
        }
    }

    fn tick(&mut self) {
        if self.current.is_empty() {
            self.put_block();
        } else if self.time.elapsed().unwrap()
            > std::time::Duration::from_millis((1000 - self.score) as u64)
        {
            self.down();
            self.time = std::time::SystemTime::now();
        }
    }

    fn put_block(&mut self) {
        self.stopped = !self.try_move(Block::new(BOARD_WIDTH / 2, BOARD_HEIGHT - 1));
    }

    fn try_move(&mut self, block: Block) -> bool {
        for i in 0..4 {
            let (x, y) = block.point(i);
            if x < 0 || x >= BOARD_WIDTH || y < 0 || y >= BOARD_HEIGHT {
                return false;
            }
            if self.board[index_at(x, y)] != Tetromino::X {
                return false;
            }
        }
        self.current = block;
        true
    }

    fn down(&mut self) {
        if !self.try_move(self.current.down()) {
            self.block_dropped();
        }
    }

    fn drop_down(&mut self) {
        while self.current.y > 0 {
            if !self.try_move(self.current.down()) {
                break;
            }
        }
        self.block_dropped();
    }

    fn block_dropped(&mut self) {
        for i in 0..4 {
            let (x, y) = self.current.point(i);
            self.board[index_at(x, y)] = self.current.kind;
        }
        self.remove_complete_lines();
        if self.current.is_empty() {
            self.put_block();
        }
    }

    fn key_pressed(&mut self, key: Key) {
        if self.stopped || self.current.is_empty() {
            return;
        }
        match key {
            Key::LEFT => {
                self.try_move(self.current.left());
            }
            Key::RIGHT => {
                self.try_move(self.current.right());
            }
            Key::UP => {
                self.try_move(self.current.rotate_right());
            }
            Key::DOWN => {
                self.try_move(self.current.rotate_left());
            }
            Key::OTHER => {
                self.down();
            }
            Key::SP => {
                self.drop_down();
            }
        };
    }

    fn remove_complete_lines(&mut self) {
        let mut line_count = 0;
        for y in (0..BOARD_HEIGHT).rev() {
            let mut complete = true;
            for x in 0..BOARD_WIDTH {
                if self.board[index_at(x, y)] == Tetromino::X {
                    complete = false;
                    break;
                }
            }
            if complete {
                line_count += 1;
                for dy in y..BOARD_HEIGHT - 1 {
                    for x in 0..BOARD_WIDTH {
                        self.board[index_at(x, dy)] = self.board[index_at(x, dy + 1)];
                    }
                }
            }
            self.score += line_count * line_count;
            self.current = Block::empty();
        }
    }

    fn draw(&self, pixmap: &mut Pixmap) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                Game::draw_square(pixmap, x, y, self.board[index_at(x, y)]);
            }
            for i in 0..4 {
                let (x, y) = self.current.point(i);
                Game::draw_square(pixmap, x, y, self.current.kind);
            }
        }
    }

    // Write a square into a pixel map
    fn draw_square(pixmap: &mut Pixmap, x: i32, y: i32, kind: Tetromino) {
        let x = x * UNIT_SIZE;
        let y = (BOARD_HEIGHT - 1 - y) * UNIT_SIZE;
        let rect = Rect::from_xywh(
            (x + 1) as f32,
            (y + 1) as f32,
            (UNIT_SIZE - 2) as f32,
            (UNIT_SIZE - 2) as f32,
        )
        .unwrap();
        let path = PathBuilder::from_rect(rect);
        let mut paint = Paint::default();
        let (r, g, b) = kind.color();
        paint.set_color_rgba8(r, g, b, 255);
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::EvenOdd,
            Transform::identity(),
            None,
        );
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(
            BOARD_WIDTH * UNIT_SIZE,
            BOARD_HEIGHT * UNIT_SIZE,
        ))
        .with_title("Titlis")
        .build(&event_loop)
        .unwrap();

    // Prepare to use softbuffer and get surface
    let window = std::rc::Rc::new(window);
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let mut game = Game::new();

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => elwt.exit(),
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { event, .. },
            ..
        } if event.state.is_pressed() => {
            match event.logical_key {
                Named(NamedKey::ArrowRight) => game.key_pressed(Key::RIGHT),
                Named(NamedKey::ArrowLeft) => game.key_pressed(Key::LEFT),
                Named(NamedKey::ArrowDown) => game.key_pressed(Key::DOWN),
                Named(NamedKey::ArrowUp) => game.key_pressed(Key::UP),
                Named(NamedKey::Space) => game.key_pressed(Key::SP),
                _ => game.key_pressed(Key::OTHER),
            };
            window.request_redraw();
        }
        Event::AboutToWait => {
            if !game.stopped {
                game.tick();
                window.set_title(format!("Titlis:{}", game.score).as_str());
                window.request_redraw();
            }
        }
        Event::WindowEvent {
            window_id,
            event: WindowEvent::RedrawRequested,
        } if window_id == window.id() => {
            // Get current window size
            let (width, height) = {
                let size = window.inner_size();
                (size.width, size.height)
            };

            // Resize surface as the window's size
            surface
                .resize(
                    core::num::NonZeroU32::new(width).unwrap(),
                    core::num::NonZeroU32::new(height).unwrap(),
                )
                .unwrap();

            // Generate a pixel buffer to draw blocks
            let mut pixmap = Pixmap::new(width, height).unwrap();

            game.draw(&mut pixmap);

            // Apply them to display buffer
            let mut buffer = surface.buffer_mut().unwrap();
            for index in 0..(width * height) as usize {
                buffer[index] = pixmap.data()[index * 4 + 2] as u32
                    | (pixmap.data()[index * 4 + 1] as u32) << 8
                    | (pixmap.data()[index * 4 + 0] as u32) << 16;
            }
            buffer.present().unwrap();
        }
        _ => (),
    });
}
