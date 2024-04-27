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

fn main() {
    println!("Hello, world!");
}
