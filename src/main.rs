use getch_rs::{Getch, Key};
use std::{thread, time};

#[derive(Clone, Copy)]
enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

type BlockShape = [[usize; 4]; 4];

struct Pos {
    x: usize,
    y: usize,
}

const BLOCKS: [BlockShape; 7] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]], // I
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]], // O
    [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]], // S
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [0, 0, 0, 0]], // Z
    [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]], // J
    [[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]], // L
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]], // T
];

const FIELD_WIDTH: usize = 13;
const FIELD_HEIGHT: usize = 21;
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

fn is_collision(field: &Field, pos: &Pos, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    return false;
}

fn draw(field: &Field, pos: &Pos) {
    let mut field_with_block = field.clone();
    for y in 0..4 {
        for x in 0..4 {
            field_with_block[y + pos.y][x + pos.x] = BLOCKS[BlockKind::I as usize][y][x] as usize;
        }
    }

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

fn main() {
    let field: Field = [
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
    ];

    let mut pos = Pos { x: 4, y: 0 };
    let g = Getch::new();

    loop {
        println!("\x1b[2J\x1b[H\x1b[?25l");

        draw(&field, &pos);

        let new_pos = Pos {
            x: pos.x,
            y: pos.y + 1,
        };

        if !is_collision(&field, &new_pos, BlockKind::I) {
            pos = new_pos
        }

        // thread::sleep(time::Duration::from_millis(100));
        match g.getch() {
            Ok(Key::Left) => {
                let new_pos = Pos {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos
                }
            }
            Ok(Key::Right) => {
                let new_pos = Pos {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos
                }
            }
            Ok(Key::Down) => {
                let new_pos = Pos {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    pos = new_pos
                }
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }
    println!("\x1b[?25h");
}
