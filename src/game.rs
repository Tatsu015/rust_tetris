use crate::block::{BlockKind, BLOCKS};

pub const FIELD_WIDTH: usize = 13;
pub const FIELD_HEIGHT: usize = 21;
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn init() -> Pos {
        Pos { x: 4, y: 0 }
    }
}

pub struct Game {
    pub field: Field,
    pub pos: Pos,
    pub block: BlockKind,
}

impl Game {
    pub fn init() -> Game {
        Game {
            field: [
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
            pos: Pos::init(),
            block: rand::random::<BlockKind>(),
        }
    }
}

pub fn is_collision(Game { field, pos, block }: &Game) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] & BLOCKS[*block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    return false;
}

pub fn draw(Game { field, pos, block }: &Game) {
    let mut field_with_block = field.clone();
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field_with_block[y + pos.y][x + pos.x] = 1
            }
        }
    }

    println!("\x1b[H");
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_with_block[y][x] == 1 {
                print!("■ ")
            } else {
                print!(". ")
            }
        }
        println!()
    }
}
