/// A coordinate point on the Sudoku board. This implementation compresses the x and
/// y coordinates down into 1 u8, because each is less than 16.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    point: u8,
}

/// Public API for `Point`
impl Point {
    /// Creates a new point. Will be invalid if beyond an index of 8.
    pub fn new(x: u8, y: u8) -> Self {
        debug_assert!(x < 9);
        debug_assert!(y < 9);

        Self {
            point: (x << 4) | (y & 0x0f),
        }
    }

    /// Gets the x-coordinate of the point.
    pub fn x(&self) -> u8 {
        self.point >> 4
    }

    /// Gets the y-coordinate of the point.
    pub fn y(&self) -> u8 {
        self.point & 0x0f
    }
}

/// Tests for this object.
mod tests {
    #![cfg(test)]

    use super::Point;

    /// Test every possible valid point and make sure it is correct.
    #[test]
    fn test_point() {
        for x in 0..9 {
            for y in 0..9 {
                let point = Point::new(x, y);

                assert_eq!(point.x(), x);
                assert_eq!(point.y(), y);
            }
        }
    }
}
