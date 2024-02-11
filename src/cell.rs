use crossterm::queue;
use crossterm::style::{
    Attribute::{Bold, Italic},
    Color::*,
    Print, ResetColor, SetAttribute, SetForegroundColor,
};
use std::io::{Error, Stdout};

#[derive(Default, Clone, PartialEq)]
pub enum CellType {
    #[default]
    Empty,
    Mine,
    Neighbouring(u8),
}

#[derive(Default, Clone, PartialEq)]
pub enum CellState {
    #[default]
    Hidden,
    // #[default]
    Open,
    Flagged,
}

#[derive(Default, Clone)]
pub struct Cell {
    pub ty: CellType,
    pub state: CellState,
}

impl Cell {
    pub fn write(&self, stdout: &mut Stdout) -> Result<(), Error> {
        match self.state {
            CellState::Hidden => queue!(
                stdout,
                SetForegroundColor(White),
                SetAttribute(Bold),
                Print("X")
            )?,
            CellState::Flagged => queue!(
                stdout,
                SetForegroundColor(Rgb {
                    r: 255,
                    g: 127,
                    b: 80
                }),
                SetAttribute(Bold),
                SetAttribute(Italic),
                Print("F")
            )?,
            CellState::Open => match self.ty {
                CellType::Empty => queue!(stdout, SetForegroundColor(DarkGrey), Print("#"))?,
                CellType::Mine => queue!(
                    stdout,
                    SetForegroundColor(Red),
                    SetAttribute(Bold),
                    SetAttribute(Italic),
                    Print("B")
                )?,
                CellType::Neighbouring(k) => {
                    queue!(
                        stdout,
                        SetForegroundColor(match k {
                            1 => Blue,
                            2 => Green,
                            3 => Yellow,
                            4 => Magenta,
                            5 => DarkRed,
                            6 => DarkRed,
                            7 => DarkRed,
                            8 => DarkRed,
                            _ => unreachable!(),
                        }),
                        Print(format!("{}", k))
                    )?;
                }
            },
        }
        queue!(stdout, ResetColor)?;

        Ok(())
    }
}
