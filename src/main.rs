mod board;
mod cell;
mod game;

use crate::game::Game;

fn main() -> std::io::Result<()> {
    let mut game = Game::new(15, 13, 10, 12)?;
    game.run()?;
    Ok(())
}
