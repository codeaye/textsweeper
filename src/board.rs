use crate::cell::{Cell, CellState, CellType};
use crossterm::{cursor::MoveToNextLine, queue, style::Print};
use rand::{thread_rng, Rng};
use std::io::{Error, Stdout};

pub type Coord = (usize, usize);

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub resolution: usize,
    pub num_mines: usize,
    pub num_allowed_flags: usize,
    pub num_used_flags: usize,
    pub mines_flagged: usize,
    pub mines_locs: Vec<usize>,
    data: Vec<Cell>,
}

impl Board {
    pub fn new(width: usize, height: usize, num_mines: usize, num_flags: usize) -> Self {
        let resolution = width * height;
        Self {
            data: vec![Cell::default(); resolution],
            num_mines: num_mines.min(resolution),
            num_allowed_flags: num_flags.max(num_mines),
            num_used_flags: 0,
            mines_flagged: 0,
            mines_locs: Vec::with_capacity(num_mines),
            resolution,
            width,
            height,
        }
    }

    pub fn reset(&mut self) {
        self.data = vec![Cell::default(); self.resolution];
        self.num_used_flags = 0;
        self.mines_flagged = 0;
        self.mines_locs.clear();
    }

    pub fn start_at(&mut self, pos: Coord) {
        let pos = self.coord_to_pos(pos);
        let mut thread_rng = thread_rng();

        while self.mines_locs.len() < self.num_mines {
            let i = thread_rng.gen_range(0..self.resolution);
            if !self.mines_locs.contains(&i) && i != pos {
                self.data[i].ty = CellType::Mine;
                self.mines_locs.push(i);
            }
        }

        self.report_mine();
    }

    fn report_mine(&mut self) {
        for mine in &self.mines_locs {
            for neighbour in self.get_neighbours(*mine) {
                match self.data[neighbour] {
                    Cell {
                        ty: CellType::Empty,
                        ..
                    } => self.data[neighbour].ty = CellType::Neighbouring(1),
                    Cell {
                        ty: CellType::Neighbouring(t),
                        ..
                    } => self.data[neighbour].ty = CellType::Neighbouring(t + 1),
                    _ => (),
                }
            }
        }
    }

    fn get_neighbours(&self, index: usize) -> Vec<usize> {
        let x = index % self.width;
        let y = index / self.width;
        let mut neighbours = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the cell itself
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                    let neighbour_index = (ny as usize * self.width) + nx as usize;
                    neighbours.push(neighbour_index);
                }
            }
        }

        neighbours
    }

    pub fn open(&mut self, index: usize) -> bool {
        let curr = &self.data[index];
        match curr {
            Cell {
                state: CellState::Hidden,
                ty: CellType::Empty,
            } => {
                self.data[index].state = CellState::Open;
                for neighbour in self.get_neighbours(index) {
                    if self.open(neighbour) {
                        return true;
                    }
                }
            }
            Cell {
                state: CellState::Hidden,
                ty: CellType::Neighbouring(_),
            } => {
                self.data[index].state = CellState::Open;
            }

            Cell {
                state: CellState::Hidden,
                ty: CellType::Mine,
            } => {
                self.data[index].state = CellState::Open;
                return true;
            }

            _ => (),
        }

        false
    }

    pub fn flag(&mut self, index: usize) {
        match &self.data[index].state {
            CellState::Flagged => {
                self.data[index].state = CellState::Hidden;
                self.num_used_flags -= 1;
                if self.mines_locs.contains(&index) {
                    self.mines_flagged -= 1;
                }
            }
            CellState::Hidden if self.num_used_flags < self.num_allowed_flags => {
                self.data[index].state = CellState::Flagged;
                self.num_used_flags += 1;
                if self.mines_locs.contains(&index) {
                    self.mines_flagged += 1;
                }
            }
            _ => (),
        }
    }

    pub fn open_all_bombs(&mut self) {
        for mine_loc in self.mines_locs.clone() {
            self.open(mine_loc);
        }
    }

    pub fn coord_to_pos(&self, pos: Coord) -> usize {
        pos.0 + self.width * pos.1
    }

    pub fn get(&self, pos: Coord) -> &Cell {
        &self.data[self.coord_to_pos(pos)]
    }

    pub fn write(&self, stdout: &mut Stdout) -> Result<(), Error> {
        for i in 0..(self.height * self.width) {
            self.data[i].write(stdout)?;
            queue!(stdout, Print(" "))?;
            if (i + 1) % self.width == 0 {
                queue!(stdout, MoveToNextLine(1))?;
            }
        }

        Ok(())
    }
}
