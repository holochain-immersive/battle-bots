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
    return a + b;
}

// Returns whether the two given position are the same position
pub fn are_positions_equal(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    return x1 == x2 && y1 == y2;
}

// Returns whether the position (x, y) is inside the map bounds
// eg. is_position_inside_map_bounds(0, 1, 2, 2) == true, is_position_inside_map_bounds(2, 1, 2, 2) == false
pub fn is_position_inside_map_bounds(
    x: usize,
    y: usize,
    map_width: usize,
    map_height: usize,
) -> bool {
    let x_inside_bounds: bool = x < map_width;  // no need to check negativity for usize
    let y_inside_bounds: bool = y < map_height; // no need to check negativity for usize
    return x_inside_bounds && y_inside_bounds;
}

// If n is a positive integer, returns n
// if n is a negative integer, returns -n
pub fn absolute(n: isize) -> usize {
    if n < 0 {
        return -n as usize;  
    }
    return n as usize;
}

// Returns the distance from one position to another, counting the number of non-diagonal steps between them
// eg. distance(0, 0, 1, 1) == 2
pub fn distance(from_pos_x: usize, from_pos_y: usize, to_pos_x: usize, to_pos_y: usize) -> usize {
    // let calculate_distance: 
    // let distance_vertical: usize = absolute(from_pos_x, to_pos_x);

}

// Returns the position that's adjacent to the left of the given one, in the form (x, y)
// eg. adjacent_position_to_the_left(4, 5) == (3, 5)
pub fn adjacent_position_to_the_left(x: usize, y: usize) -> (usize, usize) {
    (0, 0)
}
