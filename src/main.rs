mod block;
mod game;

use game::{draw, is_collision, Game, Position};
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::{thread, time};

use crate::game::{erase_line, fix_block, move_block, spawn_block};

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));

    println!("\x1b[2J\x1b[H\x1b[?25l");

    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(500));

            let mut game = game.lock().unwrap();
            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            if !is_collision(&game.field, &new_pos, game.block) {
                game.pos = new_pos
            } else {
                fix_block(&mut game);
                erase_line(&mut game.field);
                spawn_block(&mut game);
            }
            draw(&game);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();

                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }
    println!("\x1b[?25h");
}
