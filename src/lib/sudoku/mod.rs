pub mod points;

use self::points::Point;

/// This structure represents a standard 9x9 sudoku board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sudoku {
    /// The sudoku board, with each successive row being the subsequent span of 9 numbers.
    board: [u8; 81],
}

/// Private API for the sudoku board.
impl Sudoku {
    /// Get all the numbers in the column some number occupies
    fn column_numbers(point: Point) -> Vec<u8> {
        todo!()
    }
}
