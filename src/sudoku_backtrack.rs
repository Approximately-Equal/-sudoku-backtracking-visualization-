use std::fmt;
use std::{thread, time};
use colored::*;
use crate::BACKTRACK_TIME_STEP_MS;

pub struct Sudoku {
    original_board: [u8; 81],
    solvable_board: [u8; 81],
    // for visualization only
    current_cell: i8,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write top horizontal bar
        write!(f, "{}", "┏━━━━━━━┯━━━━━━━┯━━━━━━━┓ \n".black())?;
        for (r, row) in self.solvable_board.chunks(9).enumerate() {
            // Write first vertical bar | on left
            write!(f, "{}", "┃ ".black())?;
            // Write each cell in the row
            for (c, &cell) in row.iter().enumerate() {
                // Format cell text with appropriate color adn write it
                if cell == 0
                {
                    let colored_cell = format!("{:1}", cell).black();
                    write!(f, "{}", colored_cell)?;
                } else if Sudoku::to_index(r as u8, c as u8) == self.current_cell as u8
                {
                    let colored_cell = format!("{:1}", cell).yellow();
                    write!(f, "{}", colored_cell)?;
                } else if self.get_original_cell(Sudoku::to_index(r as u8, c as u8)) != 0
                {
                    let colored_cell = format!("{:1}", cell).green();
                    write!(f, "{}", colored_cell)?;
                } else if Sudoku::to_index(r as u8, c as u8) < self.current_cell as u8
                {
                    let colored_cell = format!("{:1}", cell).blue();
                    write!(f, "{}", colored_cell)?;
                } else if Sudoku::to_index(r as u8, c as u8) == self.current_cell as u8
                {
                    let colored_cell = format!("{:1}", cell).yellow();
                    write!(f, "{}", colored_cell)?;
                }
                // {
                //     let colored_cell = format!("{:1}", cell).yellow();
                //     write!(f, "{}", colored_cell)?;
                // };
                // Write vertical bar if necessary
                if (c + 1) % 3 == 0 {
                    if c < 8 {
                        write!(f, "{}", " │ ".black())?;
                    } else {
                        write!(f, "{}", " ┃".black())?;
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            // Complete line, write terminal char
            write!(f, "\n")?;
            // Write middle horizontal bars
            if (r + 1) % 3 == 0 && r != 8 {
                write!(f, "{}", "┠───────┼───────┼───────┨ \n".black())?;
            }
        }
        // Write bottom horizontal bar
        write!(f, "{}", "┗━━━━━━━┷━━━━━━━┷━━━━━━━┛ \n".black())?;
        // Reset color theme
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
    fn get_solvable_cell(&self, index: u8) -> u8 {
        self.solvable_board[index as usize]
    }
    /// gets the cell in the original board at index
    fn get_original_cell(&self, index: u8) -> u8 {
        self.original_board[index as usize]
    }

    fn set_solvable_cell(&mut self, index: u8, value: u8) {
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
    fn can_increase_current_cell(&mut self) -> bool {
        if self.current_cell < 80 {
            true
        } else {
            false
        }
    }
    fn can_decrease_current_cell(&mut self) -> bool {
        if 0 < self.current_cell {
            true
        } else {
            false
        }
    }

    // VALIDATION

    /// the function that finds whether a set of cells contains more than 1 of a specific value. this is then used to validate that the addition is a valid sudoku board.
    fn is_valid_helper(&self, start_cell: u8, cell_pattern: [u8; 9], value:u8) -> bool {
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
        (1..=9).into_iter().all(|v| self.is_valid_row_for_value(index, v))
    }
    /// determines is a the col of index is valid for all numbers 1..9. (note: that this is a slow implemnetation, but it only needs to be used once, so its not particularly important)
    fn is_valid_col(&self, index: u8) -> bool {
        (1..=9).into_iter().all(|v| self.is_valid_col_for_value(index, v))
    }
    /// determines is a the col of index is valid for all numbers 1..9. (note: that this is a slow implemnetation, but it only needs to be used once, so its not particularly important)
    fn is_valid_box(&self, index: u8) -> bool {
        (1..=9).into_iter().all(|v| self.is_valid_box_for_value(index, v))
    }
    /// validates whether the current solvable board is valid
    fn is_valid_board(&self) -> bool {
        for row in 0..9 {
            if !self.is_valid_row(9*row) {
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
        let value_at_cell = self.get_solvable_cell(index);
        self.set_solvable_cell(index, value);
        if self.is_valid_row(index)
            && self.is_valid_col(index)
            && self.is_valid_box(index)
        {
            self.set_solvable_cell(index, value_at_cell);
            return true;
        }
        self.set_solvable_cell(index, value_at_cell);
        false
    }
    /// finds the next higest valid vlaue to place in some index, gives 0 if there are no more valid placements (implicitly assumes that the current board is already valid)
    fn get_next_valid_value_from_cell(&mut self, index: u8) -> u8 {
        let current_value = self.get_solvable_cell(index);
        for i in (current_value + 1)..=9 {
            if self.is_valid_placement(index, i) {
                return i;
            }
        }
        return 0; // 0 means that are no more valid values
    }
    /// the backtracking algorithm, [explanation here]
    pub fn backtrack(&mut self, visualize: bool) -> bool {
        if !self.is_valid_board() {
            println!("board has a contradiction");
            return false;
        }
        while 0 <= self.current_cell && self.current_cell < 81 {
            // if self.get_original_cell(self.current_cell as u8) != 0 {
            //     self.current_cell += 1;
            //     continue;
            // }
            let next_valid_value = self.get_next_valid_value_from_cell(self.current_cell as u8);
            if next_valid_value > 0 {
                self.set_solvable_cell(self.current_cell as u8, next_valid_value);
                match self.can_increase_current_cell() {
                    true => self.current_cell += 1,
                    false => break,
                }
                while self.get_original_cell(self.current_cell as u8) != 0 {
                    match self.can_increase_current_cell() {
                        true => self.current_cell += 1,
                        false => break,
                    }
                }
            } else {
                self.set_solvable_cell(self.current_cell as u8, 0);
                match self.can_decrease_current_cell() {
                    true => self.current_cell -= 1,
                    false => break,
                }
                while self.get_original_cell(self.current_cell as u8) != 0 {
                    match self.can_decrease_current_cell() {
                        true => self.current_cell -= 1,
                        false => break,
                    }
                }
            }
            // visualization
            if visualize {
                println!("{}", self);
                let time_to_sleep = time::Duration::from_millis(BACKTRACK_TIME_STEP_MS.into());
                thread::sleep(time_to_sleep);
                println!("\x1B[2J\x1B[1;1H");
            }
        }
        // terminal state is reached
        if self.current_cell == 80 {
            self.current_cell += 1; // do this for the visualization only
            println!("Board Solved!");
            println!("{}", self);
            true // true for solvable board
        } else if self.current_cell == 0 {
            self.current_cell -= 1; // do this for the visualization only
            println!("Board is impossible");
            println!("{}", self);
            false // false for impossible board
        } else {
            panic!("Escaped main loop without correctly terminating");
        }
    }
}


// // implementation for visualization on the terminal
// impl Sudoku {
//     pub fn backtrack_visualize(&mut self) -> bool {
//         if !self.is_valid_board() {
//             println!("board has a contradiction");
//             println!("{}", self);
//             false
//         }
//         while 0 <= self.current_cell && self.current_cell < 81 {
//             if self.get_solvable_cell(self.current_cell as u8) != 0 {
//                 self.current_cell += 1;
//                 continue;
//             }
//             let next_valid_value = self.get_next_valid_value_from_cell(self.current_cell as u8);
//             if next_valid_value > 0 {
//                 self.set_solvable_cell(self.current_cell as u8, next_valid_value);
//             } else {
//                 self.set_solvable_cell(self.current_cell as u8, 0);
//                 self.current_cell -= 1;
//             }
//             // visualization
//             println!("{}", self);
//             let one_second = time::Duration::from_millis(BACKTRACK_TIME_STEP_MS.into());
//             thread::sleep(one_second);
//             println!("\x1B[2J\x1B[1;1H");
//         }
//         // terminal state is reached
//         if self.current_cell == 81 {
//             println!("Board Solved!");
//             println!("{}", self);
//             true // true for solvable board
//         } else if self.current_cell == -1 {
//             println!("Board is impossible");
//             println!("{}", self);
//             false // false for impossible board
//         } else {
//             panic!("Escaped main loop without correctly terminating");
//         }
//     }
// }
