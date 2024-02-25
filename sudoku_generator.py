import random

def generate_sudoku_board():
  """Generates a random Sudoku board.

  Returns:
    A 9x9 array representing a Sudoku board.
  """

  board = [[0 for _ in range(9)] for _ in range(9)]

  # Fill the board with random numbers.
  for row in range(9):
    for col in range(9):
      board[row][col] = random.randint(1, 9)

  # Check if the board is valid.
  if not is_valid_sudoku_board(board):
    # If the board is not valid, generate a new one.
    return generate_sudoku_board()

  return board

def is_valid_sudoku_board(board):
  """Checks if a Sudoku board is valid.

  Args:
    board: A 9x9 array representing a Sudoku board.

  Returns:
    True if the board is valid, False otherwise.
  """

  # Check if each row contains all the numbers from 1 to 9.
  for row in board:
    if not set(row) == set(range(1, 10)):
      return False

  # Check if each column contains all the numbers from 1 to 9.
  for col in range(9):
    if not set([board[row][col] for row in range(9)]) == set(range(1, 10)):
      return False

  # Check if each 3x3 subgrid contains all the numbers from 1 to 9.
  for row in range(0, 9, 3):
    for col in range(0, 9, 3):
      subgrid = board[row:row+3][col:col+3]
      if not set(subgrid) == set(range(1, 10)):
        return False

  return True

# Generate a random Sudoku board.
board = generate_sudoku_board()

# Print the board.
for row in board:
  print(row)
