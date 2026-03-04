use rand::{rng, RngExt};
use crate::battlesnake::Battlesnake;

pub fn simulate_game(mut snakes: Vec<Battlesnake>) -> Vec<Battlesnake> {
    snakes.sort_by(|a, b| {
        b.skill.total_cmp(&(a.skill))
    });

    snakes
}
