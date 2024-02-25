use std::fmt;

// #[derive(Debug)]
struct Sudoku {
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

impl Sudoku {
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

    fn set_solvable_cell(&mut self, row: u8, col: u8, value: u8) {
        self.solvable_board[Sudoku::to_index(row, col) as usize] = value
    }

    // note: the way these functions are is implemented, it only checks the row / column / box for duplicates of the given value. This should this is checked for every update, thus we'll always know that everything except the single addition / subtraction is valid, thus the only thing that would be invalid is the new value
    fn is_valid_row_for_value(&self, index: u8, value: u8) -> bool {
        let (row, col) = Sudoku::to_row_col(index);
        let cells_to_check = [0, 1, 2, 3, 4, 5, 6, 7, 8].iter().map(|x| x + 9 * row);
        let mut count = 0;
        for cell_index in cells_to_check {
            if self.solvable_board[cell_index as usize] == value {
                count += 1;
            }
        }
        if count <= 1 {
            return true;
        } else {
            return false;
        }
    }

    fn is_valid_col_for_value(&self, index: u8, value: u8) -> bool {
        let (row, col) = Sudoku::to_row_col(index);
        let cells_to_check = [0, 9, 18, 27, 36, 45, 54, 63, 72]
            .iter()
            .map(|x| x + (col) % 9);
        let mut count = 0;
        for cell_index in cells_to_check {
            if self.get_solvable_cell(cell_index) == value {
                count += 1;
            }
        }
        if count <= 1 {
            return true;
        } else {
            return false;
        }
    }

    fn is_valid_box_for_value(&self, index: u8, value: u8) -> bool {
        let (row, col) = Sudoku::to_row_col(index);
        let region = (col / 3) + 3 * (row / 3);
        let cells_to_check = [0, 1, 2, 9, 10, 11, 18, 19, 20]
            .iter()
            .map(|x| x + 3 * (region % 3) + 27 * (region / 3));
        let mut count = 0;
        for cell_index in cells_to_check {
            if self.get_solvable_cell(cell_index) == value {
                count += 1;
            }
        }
        if count <= 1 {
            return true;
        } else {
            return false;
        }
    }

    // helper functions for backtracking
    // (from here, use row, col because these are higher level functions, implementation should use index since it is more accurate to how the data is modelled)
    fn is_valid_placement(&self, index: u8, value: u8) -> bool {
        if self.is_valid_row_for_value(index, value)
            && self.is_valid_col_for_value(index, value)
            && self.is_valid_box_for_value(index, value)
        {
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
        let mut cell_pointer = 0;
        while 0 <= cell_pointer && cell_pointer < 81 {
            if self.get_original_cell(cell_pointer) != 0 {
                return true; //nonesense
            }
        }
        // terminal state is reached
        if cell_pointer < 81 {
            return true; // true for solvable board
        } else if cell_pointer >= 0 {
            return false;
            // false for unsolvable board
        } else {
            panic!("Somehow escaped the main body without correctly terminating");
        }
    }
}

fn main() {
    let sudoku_board: [u8; 81] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut s1 = Sudoku::new(sudoku_board);
    s1.set_solvable_cell(1, 1, 4);
    s1.set_solvable_cell(2, 1, 3);
    s1.set_solvable_cell(0, 0, 1);
    println!("{}", s1.get_next_valid_value_from_cell(1));
    // println!("{}", s1.is_valid_box_for_value(0, 4));
    println!("{}", s1);
}
