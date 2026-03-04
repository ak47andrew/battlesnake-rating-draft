use chrono::Utc;
use skillratings::glicko2::{glicko2, Glicko2Config};
use skillratings::Outcomes;
use crate::battlesnake::Battlesnake;


/// Should come in sorted form: index 0 is the 1st place, index 3 is the last
pub fn update_rating(mut snakes: Vec<Battlesnake>) -> Vec<Battlesnake> {
    let config = Glicko2Config::new();

    // Clone starting ratings
    let original: Vec<_> = snakes.iter().map(|s| s.rating.clone()).collect();

    // Accumulate results per player
    let mut new_ratings = original.clone();

    for i in 0..snakes.len() {
        for j in i+1..snakes.len() {
            let (new_i, new_j) = glicko2(
                &original[i],
                &original[j],
                &Outcomes::WIN,
                &config
            );

            new_ratings[i] = new_i;
            new_ratings[j] = new_j;
        }
    }

    for i in 0..snakes.len() {
        snakes[i].rating = new_ratings[i].clone();
        snakes[i].last_played = Utc::now();
    }

    snakes
}
