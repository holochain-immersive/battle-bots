#![allow(unused_variables)]
use battle_bots_engine::*;

mod blue;
mod grey;
mod red;
mod yellow;
mod magenta;

fn main() {
    Battle::new(vec![
        ColorConfig {
            color: Color::Blue,
            number_of_bots: 3,
            strategy: blue::blue,
        },
        ColorConfig {
            color: Color::Yellow,
            number_of_bots: 3,
            strategy: |state, position| {
                battle_bots_engine::yellow(
                    state,
                    position,
                    yellow::sum,
                    yellow::are_positions_equal,
                    yellow::is_position_inside_map_bounds,
                    yellow::absolute,
                    yellow::distance,
                    yellow::adjacent_position_to_the_left,
                )
            },
        },
        ColorConfig {
            color: Color::Grey,
            number_of_bots: 3,
            strategy: |state, position| {
                battle_bots_engine::grey(
                    state,
                    position,
                    grey::adjacent_position_in_direction,
                    grey::is_bot,
                    grey::shortest_rotation,
                    grey::rotate_direction,
                )
            },
        },
        ColorConfig {
            color: Color::Red,
            number_of_bots: 3,
            strategy: |state, position| {
                battle_bots_engine::red(
                    state,
                    position,
                    red::bot_in_position,
                    red::valid_adjacent_positions,
                    red::adjacent_positions_to_direction,
                    red::adjacent_bot,
                    red::get_closest_enemy,
                )
            },
        },
        ColorConfig {
            color: Color::Magenta,
            number_of_bots: 3,
            strategy: |state, position| magenta::magenta(state, position),
        },
    ])
    .run()
}
