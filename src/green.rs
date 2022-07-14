use battle_bots_engine::*;

// use log::LevelFilter;
// use log4rs::append::file::FileAppender;
// use log4rs::config::{Appender, Config, Root};
// use log4rs::encode::pattern::PatternEncoder;

const MY_COLOR: Color = Color::Green;

// fn rotation_string(r: Option<Rotation>) -> &'static str {
//     match r {
//         None => "none",
//         Some(rot) => match rot {
//             Rotation::Clockwise => "clockwise",
//             Rotation::Counterclockwise => "counterclockwise",
//         },
//     }
// }

/**
 * Instructions
 * =============
 *
 * At this point, you have fixed all the other bots! Hurray!
 *
 * It's now your turn to implement your own decision making algorithm!
 *
 * Do it by replacing the contents of the `blue` function.
 * - GameState: the size of the map, all the bots and all the resources and their positions
 * - Position: position of the Bot that's deciding what actuators to activate
 *
 * The `blue` function must return an `Actuators` struct, with which you can make the bot:
 * 1. Rotate the chainsaw (Option<Rotation>, if None then there is no rotation)
 * 2. Rotate the shield (Option<Rotation>, if None then there is no rotation)
 * 3. Move the bot (Option<Direction>, if None then the bot doesn't move)
 *
 * Run a battle (`cargo run`) whenever you are ready to test that the blue bot beats the other bots
 */

// what are we after?
// - attack
// - buff up
// - run away
// - chill out
enum Intent {
    Attack,
    BuffUp,
    RunAway,
    Chill,
    Guard,
}
#[derive(Debug)]
struct Target {
    pos: Position,
    weight: f32,
}

fn distance(p1: &Position, p2: &Position) -> f32 {
    let distance_x = (p2.x as isize - p1.x as isize).abs();
    let distance_y = (p2.y as isize - p1.y as isize).abs();
    ((distance_x * distance_x + distance_y * distance_y) as f32).sqrt()
}

// balance distance vs energy in the buff
fn calc_buff_weight(buff_pos: &Position, resource: &Resource, bot_position: &Position) -> f32 {
    let max_dist = 27.0;
    let dist = distance(buff_pos, &bot_position);
    // let energy_weight = resource.energy_gain as f32 / 3.0;
    let dist_weight = if dist >= max_dist {
        0.01
    } else {
        (dist as f32 * -1.0 + max_dist) / max_dist
    };
    // log::info!(" {:?} {:?}", dist, dist_weight);

    dist_weight // * energy_weight
}

fn find_buffs(game_state: &GameState, bot_position: &Position) -> Vec<Target> {
    game_state
        .resources
        .iter()
        .map(|(pos, resource)| Target {
            pos: pos.clone(),
            weight: calc_buff_weight(pos, resource, bot_position),
        })
        .collect()
}

fn best_buff(game_state: &GameState, bot_position: &Position) -> Option<Target> {
    find_buffs(game_state, bot_position)
        .into_iter()
        .reduce(|accum_target, target| {
            if target.weight > accum_target.weight {
                target
            } else {
                accum_target
            }
        })
}

// balance distance vs energy in the buff
fn calc_enemy_weight(enemy_pos: &Position, enemy_bot: &Bot, bot_position: &Position) -> f32 {
    let max_dist = 27.0;
    let dist = distance(enemy_pos, bot_position);

    // energy weight: attack weaker enemies
    let energy_weight = if enemy_bot.energy < 3 {
        1.0
    } else {
        (enemy_bot.energy as f32 * -1.0 + 21.0) / 18.0
    };

    let dist_weight = if dist >= max_dist {
        0.01
    } else {
        (dist as f32 * -1.0 + max_dist) / max_dist
    };
    // log::info!("enemy nrg {:?} : dist {:?} {:?}", energy_weight, dist, dist_weight);

    dist_weight * energy_weight
}

fn find_enemies(game_state: &GameState, bot_position: &Position) -> Vec<Target> {
    game_state
        .bots
        .iter()
        .filter(|(pos, bot)| bot.color != MY_COLOR)
        .map(|(pos, bot)| Target {
            pos: pos.clone(),
            weight: calc_enemy_weight(pos, bot, bot_position),
        })
        .collect()
}

fn best_enemy_target(game_state: &GameState, bot_position: &Position) -> Option<Target> {
    find_enemies(game_state, bot_position)
        .into_iter()
        .reduce(|accum_target, target| {
            if target.weight > accum_target.weight {
                target
            } else {
                accum_target
            }
        })
}

fn towards(bot_position: &Position, new_pos: &Position) -> Option<Direction> {
    if bot_position.x == new_pos.x && bot_position.y == new_pos.y {
        return None;
    }
    let distance_x = (new_pos.x as isize - bot_position.x as isize).abs();
    let distance_y = (new_pos.y as isize - bot_position.y as isize).abs();

    if distance_x > distance_y {
        if new_pos.x > bot_position.x {
            Some(Direction::Right)
        } else {
            Some(Direction::Left)
        }
    } else {
        if new_pos.y > bot_position.y {
            Some(Direction::Up)
        } else {
            Some(Direction::Down)
        }
    }
}
// Returns the position of the closest enemy
fn get_closest_enemy(game_state: &GameState, bot_position: &Position) -> Option<Position> {
    let closest_bot: &Position = game_state
        .bots
        .iter()
        .filter(|(pos, bot)| bot.color != MY_COLOR)
        .map(|(pos, bot)| (pos, distance(bot_position, pos)))
        .reduce(|(accum_pos, accum_dist), (pos, dist)| {
            if accum_dist < dist {
                (accum_pos, accum_dist)
            } else {
                (pos, dist)
            }
        })?
        .0;

    Some(Position {
        x: closest_bot.x,
        y: closest_bot.y,
    })
}

fn get_me(game_state: &GameState, bot_position: &Position) -> Bot {
    game_state
        .bots
        .iter()
        .find(|(pos, bot)| pos.x == bot_position.x && pos.y == bot_position.y)
        .unwrap()
        .1
}

fn rotate_towards(cur_dir: Direction, to_dir: Option<Direction>) -> Option<Rotation> {
    match (cur_dir, to_dir?) {
        (Direction::Right, Direction::Up)
        | (Direction::Up, Direction::Left)
        | (Direction::Left, Direction::Down)
        | (Direction::Down, Direction::Right) => Some(Rotation::Counterclockwise),
        (Direction::Right, Direction::Down)
        | (Direction::Down, Direction::Left)
        | (Direction::Left, Direction::Up)
        | (Direction::Up, Direction::Right) => Some(Rotation::Clockwise),
        (Direction::Up, Direction::Down)
        | (Direction::Down, Direction::Up)
        | (Direction::Left, Direction::Right)
        | (Direction::Right, Direction::Left) => Some(Rotation::Counterclockwise),
        _ => None,
    }
}

fn hack_towards(cur_dir: Direction, to_dir: Option<Direction>) -> Option<Rotation> {
    match (cur_dir, to_dir?) {
        (Direction::Right, Direction::Up)
        | (Direction::Up, Direction::Left)
        | (Direction::Left, Direction::Down)
        | (Direction::Down, Direction::Right) => Some(Rotation::Counterclockwise),
        (Direction::Right, Direction::Down)
        | (Direction::Down, Direction::Left)
        | (Direction::Left, Direction::Up)
        | (Direction::Up, Direction::Right) => Some(Rotation::Clockwise),
        (Direction::Up, Direction::Down)
        | (Direction::Down, Direction::Up)
        | (Direction::Left, Direction::Right)
        | (Direction::Right, Direction::Left) => Some(Rotation::Counterclockwise),
        _ => Some(Rotation::Clockwise),
    }
}
pub fn green(game_state: &GameState, bot_position: Position) -> Actuators {
    // let logfile = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    //     .build("output.log")
    //     .unwrap();

    // let config = Config::builder()
    //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
    //     .build(Root::builder().appender("logfile").build(LevelFilter::Info))
    //     .unwrap();

    // log4rs::init_config(config);

    let me = get_me(game_state, &bot_position);

    // SHIELD direction of shield: towards nearest enemy
    let closest_enemy = get_closest_enemy(game_state, &bot_position);
    let towards_closest_enemy = match &closest_enemy {
        Some(closest_enemy) => towards(&bot_position, &closest_enemy),
        None => None,
    };
    let shield_rotation = rotate_towards(me.shield_direction, towards_closest_enemy);

    let chainsaw_rotation = hack_towards(me.chainsaw_direction, towards_closest_enemy);
    // BUFF
    let buff_target = best_buff(game_state, &bot_position);
    // ATTACK
    let enemy_target = best_enemy_target(game_state, &bot_position);
    // if let Some(ref target) = enemy_target {
    //     log::info!("{:?}", target);
    // }

    // CHOOSE STRATEGY
    let intent = match &closest_enemy {
        Some(closest_enemy) => {
            if me.energy > 6 && distance(&bot_position, &closest_enemy) > 5.0 {
                Intent::Chill
            } else if me.energy <= 3 {
                Intent::BuffUp
            } else if enemy_target.as_ref().unwrap().weight > 0.83 {
                Intent::Attack
            } else {
                Intent::BuffUp
            }
        }
        None => Intent::Chill, // We won!
    };

    match intent {
        Intent::BuffUp => Actuators {
            move_bot: match buff_target {
                None => None,
                Some(Target { pos, .. }) => towards(&bot_position, &pos),
            },
            rotate_chainsaw: chainsaw_rotation,
            rotate_shield: shield_rotation,
        },
        Intent::Attack => Actuators {
            move_bot: match enemy_target {
                None => None,
                Some(Target { pos, .. }) => towards(&bot_position, &pos),
            },
            rotate_chainsaw: chainsaw_rotation,
            rotate_shield: shield_rotation,
        },
        _ => Actuators {
            move_bot: None,
            rotate_chainsaw: chainsaw_rotation,
            rotate_shield: shield_rotation,
        },
    }
}
