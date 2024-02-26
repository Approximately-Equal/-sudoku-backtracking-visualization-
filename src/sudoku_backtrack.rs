use crate::BACKTRACK_TIME_STEP_MS;
use colored::*;
use std::fmt;
use std::{thread, time};

pub struct Sudoku {
    start_board: [u8; 81],
    solve_board: [u8; 81],
    cell_pointer: u8,
}

enum CellType {
    Empty,
    Fixed,
    Guess,
    Check,
    Error,
}

fn get_cell_type(sudoku: &Sudoku, index: u8) -> CellType {
    if sudoku.cell_pointer == index {
        return CellType::Check;
    }
    if sudoku.get_solve_cell(index) == 0 {
        return CellType::Empty;
    }
    if sudoku.get_start_cell(index) != 0 {
        return CellType::Fixed;
    }
    if sudoku.cell_pointer > index {
        return CellType::Guess;
    }
    return CellType::Error;
}

fn color_cells(sudoku: &Sudoku, index: u8) -> ColoredString {
    let cell: &u8 = &sudoku.get_solve_cell(index);
    match get_cell_type(sudoku, index) {
        CellType::Empty => format!("{:1}", cell).black(),
        CellType::Fixed => format!("{:1}", cell).green(),
        CellType::Guess => format!("{:1}", cell).blue(),
        CellType::Check => format!("{:1}", cell).yellow(),
        CellType::Error => format!("{:1}", cell).red(),
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "┏━━━━━━━┯━━━━━━━┯━━━━━━━┓ \n".black())?;
        for row in 0..9 {
            write!(
                f,
                "{} {} {} {} {} {} {} {} {} {} {} {} {} \n",
                "┃".black(),
                color_cells(self, Sudoku::to_index(row, 0)),
                color_cells(self, Sudoku::to_index(row, 1)),
                color_cells(self, Sudoku::to_index(row, 2)),
                "│".black(),
                color_cells(self, Sudoku::to_index(row, 3)),
                color_cells(self, Sudoku::to_index(row, 4)),
                color_cells(self, Sudoku::to_index(row, 5)),
                "│".black(),
                color_cells(self, Sudoku::to_index(row, 6)),
                color_cells(self, Sudoku::to_index(row, 7)),
                color_cells(self, Sudoku::to_index(row, 8)),
                "┃".black(),
            )?;
            if (row + 1) % 3 == 0 && row < 8 {
                write!(f, "{}", "┠───────┼───────┼───────┨ \n".black())?;
            }
        }
        write!(f, "{}", "┗━━━━━━━┷━━━━━━━┷━━━━━━━┛ \n".black())?;
        colored::control::unset_override();
        Ok(())
    }
}

impl Sudoku {
    // FUNDAMENTAL FUNCTIONS

    /// initializes the sudoku
    pub fn new(sudoku_board: [u8; 81]) -> Self {
        Self {
            start_board: sudoku_board,
            solve_board: sudoku_board,
            cell_pointer: 0,
        }
    }
    /// takes a row, col and converts it into the corresponding index
    fn to_index(row: u8, col: u8) -> u8 {
        col + 9 * row
    }
    /// takes a index and converts it into the corresponding (row, col) pair
    fn to_row_col(index: u8) -> (u8, u8) {
        (index / 9, index % 9)
    }
    /// gets the cell in start board at index -index-
    fn get_start_cell(&self, index: u8) -> u8 {
        self.start_board[index as usize]
    }
    /// gets the cell in solve board at index -index-
    fn get_solve_cell(&self, index: u8) -> u8 {
        self.solve_board[index as usize]
    }
    /// sets the cell to value in solve board at index -index-
    fn set_solve_cell(&mut self, index: u8, value: u8) {
        self.solve_board[index as usize] = value;
    }
    /// check if the cell_pointer could be increased without overflow
    fn can_increase_cell_pointer(&mut self) -> bool {
        if self.cell_pointer < 80 {
            true
        } else {
            false
        }
    }
    /// check if the cell_pointer could be decreased without going underflow
    fn can_decrease_cell_pointer(&mut self) -> bool {
        if 0 < self.cell_pointer {
            true
        } else {
            false
        }
    }

    // VALIDATION

    /// takes start_index, a pattern of cell displacements from start_index, and a value, and checks whether there are at least 2 of that value in the cells in the cell pattern (hence an invalid cell_pattern for that value).
    pub fn validation_function(&self, start_cell: u8, cells_pattern: [u8; 9], value: u8) -> bool {
        cells_pattern
            .iter()
            .map(|x| x + start_cell)
            .filter(|x| self.get_solve_cell(*x) == value)
            .count()
            < 2
    }
    /// performs validation on the row of the index -index- for value -value-
    fn is_valid_row_for_value(&self, index: u8, value: u8) -> bool {
        let (row, _) = Sudoku::to_row_col(index);
        let start_cell = 9 * row;
        let cells_pattern = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        self.validation_function(start_cell, cells_pattern, value)
    }
    /// performs validation on the col of the index -index- for value -value-
    fn is_valid_col_for_value(&self, index: u8, value: u8) -> bool {
        let (_, col) = Sudoku::to_row_col(index);
        let start_cell = col;
        let cells_pattern = [0, 9, 18, 27, 36, 45, 54, 63, 72];
        self.validation_function(start_cell, cells_pattern, value)
    }
    /// performs validation on the box of the index -index- for value -value-
    fn is_valid_box_for_value(&self, index: u8, value: u8) -> bool {
        let (row, col) = Sudoku::to_row_col(index);
        let region = (col / 3) + 3 * (row / 3);
        let start_cell = 3 * (region % 3) + 27 * (region / 3);
        let cells_pattern = [0, 1, 2, 9, 10, 11, 18, 19, 20];
        self.validation_function(start_cell, cells_pattern, value)
    }
    // (note: these functions are not optimized, but it only needs to be used once, so its not particularly important)
    /// performs validation on the row of index -index- is valid for all numbers 1..9.
    fn is_valid_row(&self, index: u8) -> bool {
        (1..=9)
            .into_iter()
            .all(|v| self.is_valid_row_for_value(index, v))
    }
    /// performs validation on the col of index -index- is valid for all numbers 1..9.
    fn is_valid_col(&self, index: u8) -> bool {
        (1..=9)
            .into_iter()
            .all(|v| self.is_valid_col_for_value(index, v))
    }
    /// performs validation on the box of index -index- is valid for all numbers 1..9.
    fn is_valid_box(&self, index: u8) -> bool {
        (1..=9)
            .into_iter()
            .all(|v| self.is_valid_box_for_value(index, v))
    }
    /// performs validatation on the entire solvable board
    fn is_valid_board(&self) -> bool {
        for row in 0..9 {
            if !self.is_valid_row(9 * row) {
                return false;
            }
        }
        for col in 0..9 {
            if !self.is_valid_col(col) {
                return false;
            }
        }
        for region in 0..9 {
            if !self.is_valid_box(3 * (region % 3) + 27 * (region / 3)) {
                return false;
            }
        }
        true
    }

    // BACKTRACKING

    /// determines whether a specific value placed at some index will yeild a valid Sudoku board (implicitly assumes that the current board is already valid)
    fn is_valid_placement(&mut self, index: u8, value: u8) -> bool {
        let value_at_cell = self.get_solve_cell(index);
        self.set_solve_cell(index, value);
        if self.is_valid_row(index) && self.is_valid_col(index) && self.is_valid_box(index) {
            self.set_solve_cell(index, value_at_cell);
            return true;
        }
        self.set_solve_cell(index, value_at_cell);
        false
    }
    /// finds the next higest valid vlaue to place in some index, gives 0 if there are no more valid placements (implicitly assumes that the current board is already valid)
    fn get_next_valid_value_from_cell(&mut self, index: u8) -> u8 {
        let current_value = self.get_solve_cell(index);
        for i in (current_value + 1)..=9 {
            if self.is_valid_placement(index, i) {
                return i;
            }
        }
        0 // 0 means that are no more valid values
    }
    /// for a given iteration, determines (based on arbitrary criteria) whether that iteration should be visualized. this speeds of the visualization and makes it more interesting. can easily be turned off just returning true
    fn should_visualize_iteration(&mut self) -> bool {
        let next_valid_value = self.get_next_valid_value_from_cell(self.cell_pointer);
        let is_empty = true; // alternatively: let is_empty = self.get_solve_cell(self.cell_pointer) == 0;
        if next_valid_value > 0 && !is_empty {
            true
        } else {
            false
        }
    }
    /// the backtracking algorithm, [explanation here...]
    pub fn backtrack(&mut self, visualize: bool) -> bool {
        if !self.is_valid_board() {
            println!("board has a contradiction");
            return false;
        }
        while self.cell_pointer < 81 {
            let next_valid_value = self.get_next_valid_value_from_cell(self.cell_pointer);
            if next_valid_value > 0 {
                self.set_solve_cell(self.cell_pointer, next_valid_value);
                match self.can_increase_cell_pointer() {
                    true => self.cell_pointer += 1,
                    false => break,
                }
                while self.get_start_cell(self.cell_pointer) != 0 {
                    match self.can_increase_cell_pointer() {
                        true => self.cell_pointer += 1,
                        false => break,
                    }
                }
            } else {
                self.set_solve_cell(self.cell_pointer, 0);
                match self.can_decrease_cell_pointer() {
                    true => self.cell_pointer -= 1,
                    false => break,
                }
                while self.get_start_cell(self.cell_pointer) != 0 {
                    match self.can_decrease_cell_pointer() {
                        true => self.cell_pointer -= 1,
                        false => break,
                    }
                }
            }
            // visualization
            if visualize && self.should_visualize_iteration() {
                println!("{}", self);
                let time_to_sleep = time::Duration::from_millis(BACKTRACK_TIME_STEP_MS.into());
                thread::sleep(time_to_sleep);
                println!("\x1B[2J\x1B[1;1H");
            }
        }
        // terminal state is reached
        if self.cell_pointer == 80 {
            self.cell_pointer += 1; // do this for the visualization only
            assert!(self.is_valid_board());
            println!("Board Solved!");
            println!("{}", self);
            true // true for solvable board
        } else if self.cell_pointer == 0 {
            self.cell_pointer = 81; // do this for the visualization only
            println!("Board is impossible");
            println!("{}", self);
            false // false for unsolvable board
        } else {
            panic!("incorrect termination");
        }
    }
}
