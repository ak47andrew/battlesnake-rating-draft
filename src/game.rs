use rand::{rng, RngExt};
use crate::battlesnake::Battlesnake;

pub fn simulate_game(mut snakes: Vec<Battlesnake>) -> Vec<Battlesnake> {
    let mut generator = rng();
    snakes.sort_by(|a, b| {
        (b.skill * 3.0 + generator.random::<f64>()).total_cmp(&(a.skill * 3.0 + generator.random::<f64>()))
    });

    snakes
}
