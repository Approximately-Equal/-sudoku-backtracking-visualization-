mod sudoku_backtrack;
use sudoku_backtrack::*;

pub const BACKTRACK_TIME_STEP_MS: u32 = 100;

fn main() {
    let sudoku_board: [u8; 81] = [
        0, 3, 0,   9, 8, 4,   2, 0, 0,
        0, 0, 9,   0, 0, 7,   0, 0, 3,
        8, 0, 0,   0, 0, 0,   9, 0, 0,

        0, 0, 6,   0, 0, 2,   0, 1, 0,
        2, 0, 0,   7, 0, 5,   6, 3, 0,
        0, 0, 0,   0, 9, 0,   4, 0, 8,

        0, 6, 2,   0, 0, 0,   5, 0, 0,
        1, 0, 0,   0, 0, 0,   0, 7, 0,
        3, 0, 4,   0, 6, 1,   0, 0, 0
    ];

    let mut sudoku = Sudoku::new(sudoku_board);
    println!("Initial Board \n{}", sudoku);
    sudoku.backtrack(false);

}
