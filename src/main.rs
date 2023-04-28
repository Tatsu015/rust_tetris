mod block;

use block::{BlockKind, BLOCKS};

use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

const FIELD_WIDTH: usize = 13;
const FIELD_HEIGHT: usize = 21;
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

struct Pos {
    x: usize,
    y: usize,
}

fn is_collision(field: &Field, pos: &Pos, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    return false;
}

fn draw(field: &Field, pos: &Pos, block: BlockKind) {
    let mut field_with_block = field.clone();
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[block as usize][y][x] == 1 {
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

fn main() {
    let field = Arc::new(Mutex::new([
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
    ]));

    let pos = Arc::new(Mutex::new(Pos { x: 4, y: 0 }));
    let block = Arc::new(Mutex::new(rand::random::<BlockKind>()));

    println!("\x1b[2J\x1b[H\x1b[?25l");

    draw(
        &field.lock().unwrap(),
        &pos.lock().unwrap(),
        *block.lock().unwrap(),
    );

    {
        let pos = Arc::clone(&pos);
        let field = Arc::clone(&field);
        let block = Arc::clone(&block);
        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(500));

            let mut pos = pos.lock().unwrap();
            let mut field = field.lock().unwrap();
            let mut block = block.lock().unwrap();
            let new_pos = Pos {
                x: pos.x,
                y: pos.y + 1,
            };
            if !is_collision(&field, &new_pos, *block) {
                *pos = new_pos
            } else {
                for y in 0..4 {
                    for x in 0..4 {
                        if BLOCKS[*block as usize][y][x] == 1 {
                            field[y + pos.y][x + pos.x] = 1;
                        }
                    }
                }
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
                *pos = Pos { x: 4, y: 0 };
                *block = rand::random();
            }
            draw(&field, &pos, *block);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Pos {
                    x: pos.x.checked_sub(1).unwrap_or_else(|| pos.x),
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Pos {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let block = block.lock().unwrap();
                let new_pos = Pos {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *block) {
                    *pos = new_pos
                }
                draw(&field, &pos, *block);
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }
    println!("\x1b[?25h");
}
