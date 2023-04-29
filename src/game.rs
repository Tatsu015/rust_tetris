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
    false
}

pub fn draw(Game { field, pos, block }: &Game) {
    let mut field_with_block = *field;
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field_with_block[y + pos.y][x + pos.x] = 1
            }
        }
    }

    println!("\x1b[H");
    // for y in 0..FIELD_HEIGHT {
    for row in field_with_block.iter().take(FIELD_HEIGHT) {
        // for x in 0..FIELD_WIDTH {
        for cell in row.iter().take(FIELD_WIDTH) {
            if *cell == 1 {
                print!("■ ")
            } else {
                print!(". ")
            }
        }
        println!()
    }
}

pub fn elase_line(field: &mut Field) {
    for y in 1..FIELD_HEIGHT - 1 {
        let mut can_elase = true;
        for x in 1..FIELD_WIDTH - 1 {
            if field[y][x] == 0 {
                can_elase = false;
                break;
            }
        }
        if can_elase {
            for ty in (2..=y).rev() {
                field[ty] = field[ty - 1]
            }
        }
    }
}

pub fn fix_block(Game { field, pos, block }: &mut Game) {
    let gx = pos.x;
    let gy = pos.y;
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field[y + gy][x + gx] = 1;
            }
        }
    }
}

pub fn move_block(game: &mut std::sync::MutexGuard<Game>, new_pos: Pos) {
    if !is_collision(&*game) {
        game.pos = new_pos
    }
}
