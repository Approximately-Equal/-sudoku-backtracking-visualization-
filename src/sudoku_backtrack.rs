use std::fmt;

#[allow(dead_code)]
pub struct Sudoku {
    original_board: [u8; 81],
    solvable_board: [u8; 81],
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|-------+-------+-------|\n")?;
        for (i, &cell) in self.solvable_board.iter().enumerate() {
            if (i) % 9 == 0 {
                write!(f, "| ")?;
            }
            write!(f, "{:1} ", cell)?;
            if (i + 1) % 3 == 0 {
                write!(f, "| ")?;
            }
            if (i + 1) % 9 == 0 {
                write!(f, "\n")?;
            }
            if (i + 1) % 27 == 0 {
                write!(f, "|-------+-------+-------|\n")?;
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl Sudoku {
    // FUNDAMENTAL OPERATIONS

    pub fn new(board: [u8; 81]) -> Self {
        Sudoku {
            original_board: board,
            solvable_board: board,
        }
    }

    fn to_index(row: u8, col: u8) -> u8 {
        col + 9 * row
    }

    fn to_row_col(index: u8) -> (u8, u8) {
        (index / 9, index % 9)
    }

    fn get_solvable_cell(&self, index: u8) -> u8 {
        self.solvable_board[index as usize]
    }

    fn get_original_cell(&self, index: u8) -> u8 {
        self.solvable_board[index as usize]
    }

    fn set_solvable_cell(&mut self, index: u8, value: u8) {
        self.solvable_board[index as usize] = value
    }

    // VALIDATION

    fn is_valid_function(&self, start_cell: u8, cell_pattern: [u8; 9]) -> bool {
        let mut cells = cell_pattern
            .iter()
            .map(|x| self.get_solvable_cell(x + 9 * row))
            .collect::<Vec<u8>>();
        cells.sort();
        for i in 0..8 {
            if cells[i] != 0 && cells[i] == cells[i + 1] {
                return false;
            }
        }
        return true;
    }

    fn is_valid_row(&self, index: u8) -> bool {
        let (row, _) = Sudoku::to_row_col(index);
        let start_cell = 9 * row;
        let cells_pattern = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        self.is_valid_function(start_cell, cells_pattern)
    }

    fn is_valid_col(&self, index: u8) -> bool {
        let (_, col) = Sudoku::to_row_col(index);
        let start_cell = col;
        let cells_pattern = [0, 9, 18, 27, 36, 45, 54, 63, 72];
        self.is_valid_function(start_cell, cells_pattern)
    }

    fn is_valid_box(&self, index: u8) -> bool {
        let (row, col) = Sudoku::to_row_col(index);
        let region = (col / 3) + 3 * (row / 3);
        let start_cell = 3 * (region % 3) + 27 * (region / 3);
        let cells_pattern = [0, 1, 2, 9, 10, 11, 18, 19, 20];
        self.is_valid_function(start_cell, cells_pattern)
    }

    // BACKTRACKING

    fn is_valid_placement(&self, index: u8, value: u8) -> bool {
        if self.is_valid_row(index)
            && self.is_valid_col(index)
            && self.is_valid_box(index) {
            return true;
        }
        return false;
    }

    fn get_next_valid_value_from_cell(&self, index: u8) -> u8 {
        let current_value = self.get_solvable_cell(index);
        for i in (current_value + 1)..=9 {
            if self.is_valid_placement(index, i) {
                return i;
            }
        }
        return 0; // 0 means that are no more valid values
    }

    pub fn backtrack(&mut self) -> bool {
        let mut cell_pointer: i8 = 0;
        while 0 <= cell_pointer && cell_pointer < 81 {
            if self.get_original_cell(cell_pointer as u8) != 0 {
                cell_pointer += 1;
                continue;
            }
            let next_valid_value = self.get_next_valid_value_from_cell(cell_pointer as u8);
            if next_valid_value > 0 {
                self.set_solvable_cell(cell_pointer as u8, next_valid_value)
            } else {
                self.set_solvable_cell(cell_pointer as u8, 0);
                cell_pointer -= 1;
            }
        }
        // terminal state is reached
        if cell_pointer == 81 {
            return true; // true for solvable board
        } else if cell_pointer == -1 {
            return false; // false for impossible board
        } else {
            panic!("Escaped main loop without correctly terminating");
        }
    }
}
