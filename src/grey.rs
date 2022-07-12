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

/*
// GRID POSITION (x,y) DERIVED FROM 
// https://github.com/holochain-immersive/battle-bots-engine/blob/1cf09bf8fd34958e31a084d08e902b7ed9665684/src/direction.rs#L20
// Direction::Up if y + 1 < MAP_HEIGHT => (x, y + 1),
// Direction::Down if (y as isize) - 1 >= 0 => (x, y - 1),
// Direction::Right if x + 1 < MAP_WIDTH => (x + 1, y),
// Direction::Left if (x as isize) - 1 >= 0 => (x - 1, y), 

// CONCLUSION
// UP    => y + 1
// DOWN  => y - 1
// RIGHT => x + 1
// LEFT  => x - 1
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
        // UP    => y + 1
        assert_eq!(adjacent_position_in_direction(4, 5, Direction::Up), (4, 6));
        // DOWN  => y - 1
        assert_eq!(adjacent_position_in_direction(4, 5, Direction::Down), (4, 4));
        // RIGHT => x + 1
        assert_eq!(adjacent_position_in_direction(4, 5, Direction::Right), (5, 5));
        // LEFT  => x - 1
        assert_eq!(adjacent_position_in_direction(4, 5, Direction::Left), (3, 5));
    }

    // #[test]
    // fn is_bot_test() {
    //     // TODO
    // }
    
    #[test]
    fn shortest_rotation_test() {
        // TODO => make test_shortcut_clockwise and test_shortcut_counterclockwise DRY
        fn test_shortcut_clockwise(from: &Direction, to: &Direction) {
            let actual: Rotation = shortest_rotation(&from, &to);
            let mut success: bool = false;
            println!("success = {}", success); // remove to see warning. Keep to avoid warning...
            match actual {
                Rotation::Clockwise => {
                    success = true;
                },
                _ => {
                    success = false;
                },
            }
            assert_eq!(success, true);
        }
        fn test_shortcut_counterclockwise(from: &Direction, to: &Direction) {
            let actual: Rotation = shortest_rotation(&from, &to);
            let mut success: bool = false;
            println!("success = {}", success); // remove to see warning. Keep to avoid warning...
            match actual {
                Rotation::Counterclockwise => {
                    success = true;
                },
                _ => {
                    success = false;
                },
            }
            assert_eq!(success, true);
        }
        test_shortcut_clockwise(&Direction::Up, &Direction::Right);
        test_shortcut_clockwise(&Direction::Right, &Direction::Down);
        test_shortcut_clockwise(&Direction::Down, &Direction::Left);
        test_shortcut_clockwise(&Direction::Left, &Direction::Up);
        test_shortcut_counterclockwise(&Direction::Right, &Direction::Up);
        test_shortcut_counterclockwise(&Direction::Up, &Direction::Left);
        test_shortcut_counterclockwise(&Direction::Left, &Direction::Down);
        test_shortcut_counterclockwise(&Direction::Down, &Direction::Right);
    }
    
    #[test]
    fn rotate_direction_test() {
        // TODO => make this terrible code DRY! 
        let actual: Direction = rotate_direction(&Direction::Up, &Rotation::Clockwise);
        let mut success: bool = false;
        println!("success = {}", success); // remove to see warning. Keep to avoid warning...
        match actual {
            Direction::Right => {
                success = true;
            },
            _ => {
                success = false;
            },
        }
        assert_eq!(success, true);

        let actual: Direction = rotate_direction(&Direction::Up, &Rotation::Counterclockwise);
        match actual {
            Direction::Left => {
                success = true;
            },
            _ => {
                success = false;
            },
        }
        assert_eq!(success, true);
    }
}