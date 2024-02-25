mod board;
mod cell;
mod game;

use crate::game::Game;

fn main() -> std::io::Result<()> {
    let (mut width, mut height, mut mines) = (9, 9, 10);
    if let Some(mode) = std::env::args().skip(1).next() {
        match &*mode {
            "easy" => {
                width = 9;
                height = 9;
                mines = 10
            }
            "normal" => {
                width = 16;
                height = 16;
                mines = 40
            }
            "hard" => {
                width = 30;
                height = 16;
                mines = 99
            }
            _ => (),
        }
    }

    let mut game = Game::new(width, height, mines, mines + 2)?;
    game.run()?;
    Ok(())
}
