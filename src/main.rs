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

fn main() {
    display_a_tetromino()
}

fn display_a_tetromino() {
    let tetromino = Tetromino::rand();
    for y in (-2..=2).rev() {
        print!("| ");
        for x in -2..2 {
            let mut sq = " ";
            for i in 0..4 {
                if tetromino.shape()[i][0] == x && tetromino.shape()[i][1] == y {
                    sq = "*";
                };
            }
            print!("{}", sq);
        }
        println!(" |");
    }
}
