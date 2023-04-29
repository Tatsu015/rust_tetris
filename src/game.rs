use crate::block::{BlockKind, BLOCKS};

pub const FIELD_WIDTH: usize = 13;
pub const FIELD_HEIGHT: usize = 21;
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Position {
        Position { x: 4, y: 0 }
    }
}

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockKind,
}

impl Game {
    pub fn new() -> Game {
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
            pos: Position::init(),
            block: rand::random::<BlockKind>(),
        }
    }
}

pub fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
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
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_with_block[y][x] == 1 {
                print!("■ ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

pub fn erase_line(field: &mut Field) {
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
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field[y + pos.y][x + pos.x] = 1;
            }
        }
    }
}

pub fn move_block(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, game.block) {
        game.pos = new_pos
    }
}

pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::init();
    game.block = rand::random();

    if is_collision(&game.field, &game.pos, game.block) {
        Err(())
    } else {
        Ok(())
    }
}

pub fn gameover(game: &Game) {
    draw(game);
    println!("GAMEOVER");
    println!("press q to exit");
}

pub fn quit() {
    println!("\x1b[?25h");
}
