use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
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
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(400, 200))
        .build(&event_loop)
        .unwrap();

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => elwt.exit(),
        Event::AboutToWait => {
            window.request_redraw();
        }
        Event::WindowEvent {
            window_id,
            event: WindowEvent::RedrawRequested,
        } if window_id == window.id() => {}
        _ => (),
    });
}
