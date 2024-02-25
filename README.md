# CLI Backtracking Sudoku Solver

The script is simple, you add a sudoku board (as a single array) into the main file -- there is an example given -- and then cargo run. Note that backtracking is not the most efficient method, and for certain board, can take quite a while. The main functionality is sudoku.backtrack(), where true allows you to visualize the process, and false gives just the result.

Note that I've elected not to show all iterations. See the function should_visualize_iteration() for which iterations are shown.
