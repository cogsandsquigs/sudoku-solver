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
    /// Get a single number at a point.
    fn get_number(&self, point: Point) -> u8 {
        self.board[(point.x() + point.y() * 9) as usize]
    }

    /// Get all the numbers in the column some number occupies
    fn column_numbers(&self, point: Point) -> Vec<u8> {
        todo!()
    }
}
