use crate::board::Board;
use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, SetCursorStyle, Show},
    event::{read, DisableMouseCapture, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Print, PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType},
};
use std::io::{stdout, Error, Stdout, Write};

#[derive(PartialEq)]
pub enum GameState {
    PreInit,
    Playing,
    Lost,
    Won,
}

pub struct Game {
    stdout: Stdout,
    board: Board,
    state: GameState,

    mouse_x: usize,
    mouse_y: usize,
}

impl Game {
    pub fn new(
        width: usize,
        height: usize,
        num_mines: usize,
        num_flags: usize,
    ) -> Result<Self, Error> {
        let mut stdout = stdout();
        execute!(
            stdout,
            SetCursorStyle::BlinkingUnderScore,
            DisableMouseCapture,
        )?;

        terminal::enable_raw_mode()?;

        Ok(Self {
            stdout,
            board: Board::new(width, height, num_mines, num_flags),
            mouse_x: 0,
            mouse_y: 0,
            state: GameState::PreInit,
        })
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.state = GameState::PreInit;
    }

    pub fn run(&mut self) -> Result<(), Error> {
        'main_loop: loop {
            self.draw()?;
            'event_loop: loop {
                let event = read()?;
                if let Event::Key(event) = event {
                    match event {
                        KeyEvent {
                            code: KeyCode::Esc | KeyCode::End,
                            ..
                        } => break 'main_loop,
                        k => {
                            self.key(k.code)?;
                            break 'event_loop;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn key(&mut self, event: KeyCode) -> Result<bool, Error> {
        match event {
            KeyCode::Left | KeyCode::Char('a') => {
                self.mouse_x = (self.mouse_x as isize - 2).max(0isize) as usize
            }

            KeyCode::Right | KeyCode::Char('d') => {
                self.mouse_x =
                    (self.mouse_x as isize + 2).min((self.board.width as isize - 1) * 2) as usize
            }

            KeyCode::Up | KeyCode::Char('w') => {
                self.mouse_y = (self.mouse_y as isize - 1).max(0isize) as usize
            }

            KeyCode::Down | KeyCode::Char('s') => {
                self.mouse_y = (self.mouse_y + 1).min(self.board.height - 1)
            }

            KeyCode::Char(' ') => {
                let pos = (self.mouse_x / 2, self.mouse_y);
                match self.state {
                    GameState::PreInit => {
                        self.board.start_at(pos);
                        self.board.open(self.board.coord_to_pos(pos));
                        self.state = GameState::Playing
                    }
                    GameState::Playing => {
                        let is_over = self.board.open(self.board.coord_to_pos(pos));
                        if is_over {
                            self.state = GameState::Lost;
                            self.board.open_all_bombs();
                            execute!(self.stdout, Hide)?;
                            return Ok(true);
                        }
                    }
                    _ => (),
                }
            }

            KeyCode::Char('f') if self.state == GameState::Playing => {
                self.board
                    .flag(self.board.coord_to_pos((self.mouse_x / 2, self.mouse_y)));

                if self.board.mines_flagged == self.board.num_mines {
                    self.state = GameState::Won;
                    execute!(self.stdout, Hide)?;
                    return Ok(true);
                }
            }

            KeyCode::Char('r') if self.state != GameState::PreInit => {
                self.reset();
            }

            _ => (),
        }

        Ok(false)
    }

    pub fn draw(&mut self) -> Result<(), Error> {
        execute!(self.stdout, Clear(ClearType::All), MoveTo(0, 0),)?;
        let pos = (self.mouse_x / 2, self.mouse_y);
        self.board.write(&mut self.stdout)?;
        match self.state {
            GameState::Playing | GameState::PreInit => {
                execute!(self.stdout, Show).unwrap();
                queue!(
                    self.stdout,
                    MoveToNextLine(1),
                    Print(format!(
                        "Flags Used: {}/{}",
                        self.board.num_used_flags, self.board.num_allowed_flags
                    )),
                    MoveToNextLine(1),
                    Print(format!("Selected tile: {:?} [", pos)),
                )?;
                self.board.get(pos).write(&mut self.stdout)?;
                queue!(self.stdout, Print("]"))?;
            }
            GameState::Won => {
                queue!(
                    self.stdout,
                    MoveToNextLine(1),
                    PrintStyledContent("YOU WON!".green().bold().italic()),
                    MoveToNextLine(1),
                    PrintStyledContent("Hit R to restart!".blue().bold().italic()),
                    MoveToNextLine(1),
                    PrintStyledContent("Hit ESC to quit!".white().bold().italic())
                )?;
            }
            GameState::Lost => {
                queue!(
                    self.stdout,
                    MoveToNextLine(1),
                    PrintStyledContent("YOU LOST!".red().bold().italic()),
                    MoveToNextLine(1),
                    PrintStyledContent("Hit R to restart!".blue().bold().italic()),
                    MoveToNextLine(1),
                    PrintStyledContent("Hit ESC to quit!".white().bold().italic())
                )?;
            }
        }
        self.stdout.flush()?;
        execute!(
            self.stdout,
            MoveTo(self.mouse_x as u16, self.mouse_y as u16)
        )?;
        Ok(())
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        execute!(
            self.stdout,
            SetCursorStyle::DefaultUserShape,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
        execute!(self.stdout, Show).unwrap();
    }
}
