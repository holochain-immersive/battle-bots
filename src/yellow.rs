/**
 * The yellow bot is broken! It's using all the functions below, but they seem not to be implemented correctly
 *
 * Please help us fix the bot!
 *
 * Instructions
 * =============
 *
 * Implement all the functions below
 * Run a battle (`cargo run`) after they have been implemented to test that the yellow bot works again
 */

// Return the sum of a and b
pub fn sum(a: usize, b: usize) -> usize {
    0
}

// Returns whether the two given position are the same position
pub fn are_positions_equal(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    false
}

// Returns whether the position (x, y) is inside the map bounds
// eg. is_position_inside_map_bounds(0, 1, 2, 2) == true, is_position_inside_map_bounds(2, 1, 2, 2) == false
pub fn is_position_inside_map_bounds(
    x: usize,
    y: usize,
    map_width: usize,
    map_height: usize,
) -> bool {
    false
}

// If n is a positive integer, returns n
// if n is a negative integer, returns -n
pub fn absolute(n: isize) -> usize {
    0
}

// Returns the distance from one position to another, counting the number of non-diagonal steps between them
// eg. distance(0, 0, 1, 1) == 2
pub fn distance(from_pos_x: usize, from_pos_y: usize, to_pos_x: usize, to_pos_y: usize) -> usize {
    0
}

// Returns the position that's adjacent to the left of the given one, in the form (x, y)
// eg. adjacent_position_to_the_left(4, 5) == (3, 5)
pub fn adjacent_position_to_the_left(x: usize, y: usize) -> (usize, usize) {
    (0, 0)
}






/*
 * UNIT TESTS
 */
#[cfg(test)]
mod tests {
    use crate::yellow::*;
    
    #[test]
    fn sum_test() {
        assert_eq!(sum(1,4), 5);
        assert_eq!(sum(12,1200239), 1200251);
        assert_eq!(sum(0,0), 0);
    }
    
    #[test]
    fn are_positions_equal_test() {
        // should be true
        assert_eq!(are_positions_equal(1,3,1,3), true);
        assert_eq!(are_positions_equal(24,24,24,24), true);
        assert_eq!(are_positions_equal(2,12,2,12), true);
        assert_eq!(are_positions_equal(0,0,0,0), true);
        // x or y or both mismatch
        assert_eq!(are_positions_equal(2,12,3,12), false); // x mismatch
        assert_eq!(are_positions_equal(1,1,1,0), false); // y mismatch
        assert_eq!(are_positions_equal(12,1,13,0), false); // both mismatch
        assert_eq!(are_positions_equal(1,0,12,3), false); // both mismatch
    }
    
    #[test]
    fn is_position_inside_map_bounds_test() {
        // tests taken from description
        assert_eq!(is_position_inside_map_bounds(0, 1, 2, 2), true);
        assert_eq!(is_position_inside_map_bounds(2, 1, 2, 2), false);
        // should be false for map with zero width or height
        assert_eq!(is_position_inside_map_bounds(0,0,0,0), false);
        // should be true
        assert_eq!(is_position_inside_map_bounds(0,1,2,2), true);
        // should be true (edge case)
        assert_eq!(is_position_inside_map_bounds(1,1,2,2), true);
        assert_eq!(is_position_inside_map_bounds(4,9,5,10), true);
        // should be true (zero case)
        assert_eq!(is_position_inside_map_bounds(0,0,5,5), true);
        // should be false (edge case)
        assert_eq!(is_position_inside_map_bounds(2,1,2,2), false);
        assert_eq!(is_position_inside_map_bounds(1,2,2,2), false);
    }
    
    #[test]
    fn absolute_test() {
        assert_eq!(absolute(0), 0);
        assert_eq!(absolute(12), 12);
        assert_eq!(absolute(-4), 4);
    }
    
    #[test]
    fn distance_test() {
        assert_eq!(distance(0,0,0,0), 0);
        assert_eq!(distance(5,0,1,0), 4);
        assert_eq!(distance(1,0,5,0), 4);
        assert_eq!(distance(0,1,0,5), 4);
        assert_eq!(distance(0,5,0,1), 4);
        assert_eq!(distance(1,5,5,1), 8);
        assert_eq!(distance(1,5,15,1), 18);
    }
    
    #[test]
    fn adjacent_position_to_the_left_test() {
        // test taken from description
        assert_eq!(adjacent_position_to_the_left(4, 5), (3, 5));
        // other
        assert_eq!(adjacent_position_to_the_left(5, 5), (4, 5));
        assert_eq!(adjacent_position_to_the_left(1, 1), (0, 1));
        assert_eq!(adjacent_position_to_the_left(1, 0), (0, 0));
    }
    
    #[test]
    #[should_panic]
    fn adjacent_position_to_the_left_panic_test() {
        // out of bounds should panic
        adjacent_position_to_the_left(0, 0);
    }
}
