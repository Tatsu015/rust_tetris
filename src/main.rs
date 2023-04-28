mod block;
mod game;

use block::BLOCKS;

use game::{draw, is_collision, Game, Pos, FIELD_HEIGHT, FIELD_WIDTH};
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let game = Arc::new(Mutex::new(Game::init()));

    println!("\x1b[2J\x1b[H\x1b[?25l");

    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(500));

            let mut game = game.lock().unwrap();
            let new_pos = Pos {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            if !is_collision(&game) {
                game.pos = new_pos
            } else {
                let gx = game.pos.x;
                let gy = game.pos.y;
                for y in 0..4 {
                    for x in 0..4 {
                        if BLOCKS[game.block as usize][y][x] == 1 {
                            game.field[y + gy][x + gx] = 1;
                        }
                    }
                }
                for y in 1..FIELD_HEIGHT - 1 {
                    let mut can_elase = true;
                    for x in 1..FIELD_WIDTH - 1 {
                        if game.field[y][x] == 0 {
                            can_elase = false;
                            break;
                        }
                    }
                    if can_elase {
                        for ty in (2..=y).rev() {
                            game.field[ty] = game.field[ty - 1]
                        }
                    }
                }
                game.pos = Pos::init();
                game.block = rand::random();
            }
            draw(&game);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();

                let new_pos = Pos {
                    x: game.pos.x.checked_sub(1).unwrap_or_else(|| game.pos.x),
                    y: game.pos.y,
                };
                if !is_collision(&game) {
                    game.pos = new_pos
                }
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Pos {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game) {
                    game.pos = new_pos
                }
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Pos {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                if !is_collision(&game) {
                    game.pos = new_pos
                }
                draw(&game);
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }
    println!("\x1b[?25h");
}
