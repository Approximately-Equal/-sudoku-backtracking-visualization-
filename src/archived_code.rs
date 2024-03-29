// let characters: Vec<&str> = vec![
//     "━", "┏", "┓", "┗", "┛", "┃", " ", "┠", "┨", "┯", "┷", "│", "─", "┼",
// ];

// ARCHIVES 1

// note: the way these functions are is implemented, it only checks the row / column / box for duplicates of the given value. This should this is checked for every update, thus we'll always know that everything except the single addition / subtraction is valid, thus the only thing that would be invalid is the new value
fn is_valid_row_for_value(&self, index: u8, value: u8) -> bool {
    let (row, _) = Sudoku::to_row_col(index);
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
    let (_, col) = Sudoku::to_row_col(index);
    let cells_to_check = [0, 9, 18, 27, 36, 45, 54, 63, 72]
        .iter()
        .map(|x| x + (col % 9));
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

// note: these following functions are the above but check for all values in thier respective rows / columsn / boxes
pub fn is_valid_row(&self, index: u8) -> bool {
    let (row, _) = Sudoku::to_row_col(index);
    let mut cells_to_check = [0, 1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| self.get_solvable_cell(x + 9 * row))
        .collect::<Vec<u8>>();
    cells_to_check.sort();
    for i in 0..8 {
        if cells_to_check[i] != 0 && cells_to_check[i] == cells_to_check[i + 1] {
            return false;
        }
    }
    return true;
}

pub fn is_valid_col(&self, index: u8) -> bool {
    let (_, col) = Sudoku::to_row_col(index);
    let mut cells_to_check = [0, 9, 18, 27, 36, 45, 54, 63, 72]
        .iter()
        .map(|x| self.get_solvable_cell(x + (col % 9)))
        .collect::<Vec<u8>>();
    cells_to_check.sort();
    for i in 0..8 {
        if cells_to_check[i] != 0 && cells_to_check[i] == cells_to_check[i + 1] {
            return false;
        }
    }
    return true;
}

pub fn is_valid_box(&self, index: u8) -> bool {
    let (row, col) = Sudoku::to_row_col(index);
    let region = (col / 3) + 3 * (row / 3);
    let start_cell = 3 * (region % 3) + 27 * (region / 3);
    let mut cells_to_check = [0, 1, 2, 9, 10, 11, 18, 19, 20]
        .iter()
        .map(|x| self.get_solvable_cell(x + start_cell))
        .collect::<Vec<u8>>();
    cells_to_check.sort();
    for i in 0..(9 - 1) {
        if cells_to_check[i] != 0 && cells_to_check[i] == cells_to_check[i + 1] {
            return false;
        }
    }
    return true;
}




// ARCHIVES 2

fn is_valid_helper(&self, start_cell: u8, cell_pattern: [u8; 9]) -> bool {
    let mut cells = cell_pattern
        .iter()
        .map(|x| self.get_solvable_cell(x + start_cell))
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
    self.is_valid_helper(start_cell, cells_pattern)
}

fn is_valid_col(&self, index: u8) -> bool {
    let (_, col) = Sudoku::to_row_col(index);
    let start_cell = col;
    let cells_pattern = [0, 9, 18, 27, 36, 45, 54, 63, 72];
    self.is_valid_helper(start_cell, cells_pattern)
}

fn is_valid_box(&self, index: u8) -> bool {
    let (row, col) = Sudoku::to_row_col(index);
    let region = (col / 3) + 3 * (row / 3);
    let start_cell = 3 * (region % 3) + 27 * (region / 3);
    let cells_pattern = [0, 1, 2, 9, 10, 11, 18, 19, 20];
    self.is_valid_helper(start_cell, cells_pattern)
}

/// validates whether the current solvable board is valid
pub fn is_valid_board(&self) -> bool {
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
    return true;
}


// ARCHIVES 3
//
// // impl fmt::Display for Sudoku {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     //     // write top horizontal bar
//     //     write!(f, "{}", "|-------+-------+-------|\n".black())?;
//     //     for row in 0..9 {
//     //         write!(f, "{}", "|".black())?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 0) as char)?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 1) as char)?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 2) as char)?;
//     //         write!(f, "{:1}", "|".black())?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 3) as char)?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 4) as char)?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 5) as char)?;
//     //         write!(f, "{:1}", "|".black())?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 6) as char)?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 7) as char)?;
//     //         write!(f, "{:1}", self.get_solvable_cell(9*row + 8) as char)?;
//     //         write!(f, "{:1}", "|".black())?;

//     //         if (row+1) % 3 == 0 {
//     //             write!(f, "{}", "|-------+-------+-------|\n".black())?;
//     //         }
//     //     }
//     //     Ok(())
//     // }

//         // write top horizontal bar
//         write!(f, "{}", "|-------+-------+-------|\n".black())?;
//         for (i, &cell) in self.solvable_board.iter().enumerate() {
//             // write first vertical bar | on left
//             if (i) % 9 == 0 {
//                 write!(f, "{}", "| ".black())?;
//             }
//             // write the actual numbers (with added space)
//             write!(f, "{:1} ", cell)?;
//             // write right 3 vertical bars |
//             if i % 3 == 2 {
//                 write!(f, "{}", "| ".black())?;
//             }
//             // complete line, write terminal char
//             if (i + 1) % 9 == 0 {
//                 write!(f, "\n")?;
//             }
//             // write bottom three horizontal bars
//             if (i + 1) % 27 == 0 {
//                 write!(f, "{}", "|-------+-------+-------|\n".black())?;
//             }
//         }
//         Ok(())
//     }
// }
