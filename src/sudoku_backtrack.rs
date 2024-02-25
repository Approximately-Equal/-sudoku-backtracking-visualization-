use crate::BACKTRACK_TIME_STEP_MS;
use colored::*;
use std::fmt;
use std::{thread, time};

pub struct Sudoku {
    original_board: [u8; 81],
    solvable_board: [u8; 81],
    // for visualization only
    current_cell: u8,
}

// this differentiates the kinds of cells that are possible for use in coloring them based on those types.
enum CellType {
    Empty,
    Fixed,
    Guess,
    Check,
    Error,
}

fn get_cell_type(sudoku: &Sudoku, index: u8) -> CellType {
    // add this first to ensure that it will be caught in all cases
    if sudoku.current_cell == index {
        return CellType::Check;
    };
    if sudoku.get_cell(index) == 0 {
        return CellType::Empty;
    };
    if sudoku.get_original_cell(index) != 0 {
        return CellType::Fixed;
    };
    if sudoku.current_cell > index {
        return CellType::Guess;
    };
    return CellType::Error;
}

fn write_cell_type(f: &mut fmt::Formatter<'_>, sudoku: &Sudoku, index: u8) -> fmt::Result {
    let cell: &u8 = &sudoku.get_cell(index);
    match get_cell_type(sudoku, index) {
        CellType::Empty => write!(f, "{} ", format!("{:1}", cell).black())?,
        CellType::Fixed => write!(f, "{} ", format!("{:1}", cell).green())?,
        CellType::Guess => write!(f, "{} ", format!("{:1}", cell).blue())?,
        CellType::Check => write!(f, "{} ", format!("{:1}", cell).yellow())?,
        CellType::Error => write!(f, "{} ", format!("{:1}", cell).red())?,
    }
    Ok(())
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write top horizontal bar
        write!(f, "{}", "┏━━━━━━━┯━━━━━━━┯━━━━━━━┓ \n".black())?;
        for row in 0..9 {
            // write first vertical bar | on left
            write!(f, "{}", "┃ ".black())?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 0))?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 1))?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 2))?;
            write!(f, "{}", "│ ".black())?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 3))?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 4))?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 5))?;
            write!(f, "{}", "│ ".black())?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 6))?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 7))?;
            write_cell_type(f, self, Sudoku::to_index(row as u8, 8))?;
            write!(f, "{}", "┃".black())?;
            write!(f, "\n")?;
            // write middle horizontal bars
            if (row + 1) % 3 == 0 && row != 8 {
                write!(f, "{}", "┠───────┼───────┼───────┨ \n".black())?;
            }
        }
        // write bottom horizontal bar
        write!(f, "{}", "┗━━━━━━━┷━━━━━━━┷━━━━━━━┛ \n".black())?;
        // reset color theme
        colored::control::unset_override();
        Ok(())
    }
}

#[allow(dead_code)]
impl Sudoku {
    // FUNDAMENTAL OPERATIONS

    /// initializes a new Sudoku
    pub fn new(board: [u8; 81]) -> Self {
        Sudoku {
            original_board: board,
            solvable_board: board,
            current_cell: 0,
        }
    }
    /// converts an index into its corresponding row and col
    pub fn to_index(row: u8, col: u8) -> u8 {
        col + 9 * row
    }
    /// converts a row, col pair into its corresponding indeex
    pub fn to_row_col(index: u8) -> (u8, u8) {
        (index / 9, index % 9)
    }
    /// gets the cell in the solvable board at index
    fn get_cell(&self, index: u8) -> u8 {
        self.solvable_board[index as usize]
    }
    /// gets the cell in the original board at index
    fn get_original_cell(&self, index: u8) -> u8 {
        self.original_board[index as usize]
    }
    /// set the cell to value in the solvable board at index
    fn set_cell(&mut self, index: u8, value: u8) {
        self.solvable_board[index as usize] = value
    }
    /// checks if the index is in the bounadies of the board
    pub fn is_in_bounds(index: i8) -> bool {
        if 0 <= index && index < 81 {
            true
        } else {
            false
        }
    }
    /// check if the current_cell could be increased without overflow
    fn can_increase_current_cell(&mut self) -> bool {
        if self.current_cell < 80 {
            true
        } else {
            false
        }
    }
    /// check if the current_cell could be decreased without going underflow
    fn can_decrease_current_cell(&mut self) -> bool {
        if 0 < self.current_cell {
            true
        } else {
            false
        }
    }

    // VALIDATION

    /// the function that finds whether a set of cells contains more than 1 of a specific value. this is then used to validate that the addition is a valid sudoku board.
    fn is_valid_helper(&self, start_cell: u8, cell_pattern: [u8; 9], value: u8) -> bool {
        let cells_to_check = cell_pattern.iter().map(|x| x + start_cell);
        let mut count = 0;
        for cell_index in cells_to_check {
            if self.solvable_board[cell_index as usize] == value {
                count += 1;
            }
        }
        if count <= 1 {
            true
        } else {
            false
        }
    }
    /// determines is a the row of index has less than two of value -- hence it is valid for that value.
    fn is_valid_row_for_value(&self, index: u8, value: u8) -> bool {
        let (row, _) = Sudoku::to_row_col(index);
        let start_cell = 9 * row;
        let cells_pattern = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        self.is_valid_helper(start_cell, cells_pattern, value)
    }
    /// determines is a the col of index has less than two of value -- hence it is valid for that value.
    fn is_valid_col_for_value(&self, index: u8, value: u8) -> bool {
        let (_, col) = Sudoku::to_row_col(index);
        let start_cell = col;
        let cells_pattern = [0, 9, 18, 27, 36, 45, 54, 63, 72];
        self.is_valid_helper(start_cell, cells_pattern, value)
    }
    /// determines is a the box of index has less than two of value -- hence it is valid for that value.
    fn is_valid_box_for_value(&self, index: u8, value: u8) -> bool {
        let (row, col) = Sudoku::to_row_col(index);
        let region = (col / 3) + 3 * (row / 3);
        let start_cell = 3 * (region % 3) + 27 * (region / 3);
        let cells_pattern = [0, 1, 2, 9, 10, 11, 18, 19, 20];
        self.is_valid_helper(start_cell, cells_pattern, value)
    }
    /// determines is a the row of index is valid for all numbers 1..9. (note: that this is a slow implemnetation, but it only needs to be used once, so its not particularly important)
    fn is_valid_row(&self, index: u8) -> bool {
        (1..=9)
            .into_iter()
            .all(|v| self.is_valid_row_for_value(index, v))
    }
    /// determines is a the col of index is valid for all numbers 1..9. (note: that this is a slow implemnetation, but it only needs to be used once, so its not particularly important)
    fn is_valid_col(&self, index: u8) -> bool {
        (1..=9)
            .into_iter()
            .all(|v| self.is_valid_col_for_value(index, v))
    }
    /// determines is a the col of index is valid for all numbers 1..9. (note: that this is a slow implemnetation, but it only needs to be used once, so its not particularly important)
    fn is_valid_box(&self, index: u8) -> bool {
        (1..=9)
            .into_iter()
            .all(|v| self.is_valid_box_for_value(index, v))
    }
    /// validates whether the current solvable board is valid
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
        let value_at_cell = self.get_cell(index);
        self.set_cell(index, value);
        if self.is_valid_row(index) && self.is_valid_col(index) && self.is_valid_box(index) {
            self.set_cell(index, value_at_cell);
            return true;
        }
        self.set_cell(index, value_at_cell);
        false
    }
    /// finds the next higest valid vlaue to place in some index, gives 0 if there are no more valid placements (implicitly assumes that the current board is already valid)
    fn get_next_valid_value_from_cell(&mut self, index: u8) -> u8 {
        let current_value = self.get_cell(index);
        for i in (current_value + 1)..=9 {
            if self.is_valid_placement(index, i) {
                return i;
            }
        }
        return 0; // 0 means that are no more valid values
    }

    fn should_visualize_iteration(&mut self) -> bool {
        let next_valid_value = self.get_next_valid_value_from_cell(self.current_cell);
        let is_empty = match get_cell_type(self, self.current_cell) {
            CellType::Empty => true,
           _ => false
        };
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
        while self.current_cell < 81 {
            let next_valid_value = self.get_next_valid_value_from_cell(self.current_cell);
            if next_valid_value > 0 {
                self.set_cell(self.current_cell, next_valid_value);
                match self.can_increase_current_cell() {
                    true => self.current_cell += 1,
                    false => break,
                }
                while self.get_original_cell(self.current_cell) != 0 {
                    match self.can_increase_current_cell() {
                        true => self.current_cell += 1,
                        false => break,
                    }
                }
            } else {
                self.set_cell(self.current_cell, 0);
                match self.can_decrease_current_cell() {
                    true => self.current_cell -= 1,
                    false => break,
                }
                while self.get_original_cell(self.current_cell) != 0 {
                    match self.can_decrease_current_cell() {
                        true => self.current_cell -= 1,
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
        if self.current_cell == 80 {
            self.current_cell += 1; // do this for the visualization only
            assert!(self.is_valid_board());
            println!("Board Solved!");
            println!("{}", self);
            true // true for solvable board
        } else if self.current_cell == 0 {
            self.current_cell = 81; // do this for the visualization only
            println!("Board is impossible");
            println!("{}", self);
            false // false for unsolvable board
        } else {
            panic!("incorrect termination");
        }
    }
}
