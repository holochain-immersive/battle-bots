use battle_bots_engine::*;

/** 
 * The grey bot is broken! It's using all the functions below, but they seem not to be implemented correctly
 * 
 * Please help us fix the bot!
 * 
 * Instructions
 * =============
 * 
 * Implement all the functions below
 * Run a battle (`cargo run`) after they have been implemented to test that the grey bot works again
 */


// Returns the position that's adjacent to the given one in the given direction, in the form (x, y)
// eg. adjacent_position_in_direction(4, 5, Direction::Down) == (4, 6)
pub fn adjacent_position_in_direction(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    (0, 0)
}

// Returns whether there is a bot in the given position
pub fn is_bot(game_state: &GameState, position: &Position) -> bool {
    false
}

// Returns the shortest way to rotate the "from" direction to get the "to" direction
// Assumes that from and to are not equal
// eg. shortest_rotation(Direction::Up, Direction::Right) == Rotation::Clockwise
pub fn shortest_rotation(from: &Direction, to: &Direction) -> Rotation {
    Rotation::Clockwise
}

// Rotate the given direction with the given rotation
// eg. rotate_direction(Direction::Up, Rotation::Clockwise) == Direction::Right
pub fn rotate_direction(direction: &Direction, rotation: &Rotation) -> Direction {
    Direction::Down
}





// ADD UNIT TESTS
#[cfg(test)]
mod tests {
    use crate::grey::*;

    #[test]
    fn adjacent_position_in_direction_test() {
        fn test_helper(x: usize, y: usize, dir: Direction, x_new: usize, y_new: usize) {
            assert_eq!(adjacent_position_in_direction(x, y, dir), (x_new, y_new));
        }   
        test_helper(4, 5, Direction::Up, 4, 4);    // UP    => y + 1 => FIX: y - 1 (by Guillem)
        test_helper(4, 5, Direction::Down, 4, 6);  // DOWN  => y - 1 => FIX: y + 1 (by Guillem)
        test_helper(4, 5, Direction::Right, 5, 5); // RIGHT => x + 1
        test_helper(4, 5, Direction::Left, 3, 5);  // LEFT  => x - 1
    }

    // #[test]
    // fn is_bot_test() {
    //     // TODO
    // }

    #[test]
    fn shortest_rotation_test() {
        fn test_helper(from: &Direction, to: &Direction, target: Rotation) {
            let actual: Rotation = shortest_rotation(&from, &to);
            let success: bool = match (actual, target) {
                (Rotation::Clockwise, Rotation::Clockwise) => true,
                (Rotation::Counterclockwise, Rotation::Counterclockwise) => true,
                _ => false,
            };
            assert_eq!(success, true);
        }
        // clockwise cases
        test_helper(&Direction::Up, &Direction::Right, Rotation::Clockwise);
        test_helper(&Direction::Right, &Direction::Down, Rotation::Clockwise);
        test_helper(&Direction::Down, &Direction::Left, Rotation::Clockwise);
        test_helper(&Direction::Left, &Direction::Up, Rotation::Clockwise);
        // counter clockwise cases
        test_helper(&Direction::Right, &Direction::Up, Rotation::Counterclockwise);
        test_helper(&Direction::Up, &Direction::Left, Rotation::Counterclockwise);
        test_helper(&Direction::Left, &Direction::Down, Rotation::Counterclockwise);
        test_helper(&Direction::Down, &Direction::Right, Rotation::Counterclockwise);
    }

    #[test]
    fn rotate_direction_test() {
        fn test_helper(direction: &Direction, rotation: &Rotation, target: Direction) {
            let actual: Direction = rotate_direction(&direction, &rotation);
            let success: bool = match (actual, target) {
                (Direction::Up, Direction::Up) => true,
                (Direction::Down, Direction::Down) => true,
                (Direction::Right, Direction::Right) => true,
                (Direction::Left, Direction::Left) => true,
                _ => false,
            };
            assert_eq!(success, true);
        }
        test_helper(&Direction::Up, &Rotation::Clockwise, Direction::Right);
        test_helper(&Direction::Up, &Rotation::Counterclockwise, Direction::Left);
        test_helper(&Direction::Down, &Rotation::Clockwise, Direction::Left);
        test_helper(&Direction::Down, &Rotation::Counterclockwise, Direction::Right);
        test_helper(&Direction::Right, &Rotation::Clockwise, Direction::Down);
        test_helper(&Direction::Right, &Rotation::Counterclockwise, Direction::Up);
        test_helper(&Direction::Left, &Rotation::Clockwise, Direction::Up);
        test_helper(&Direction::Left, &Rotation::Counterclockwise, Direction::Down);
    }
}
