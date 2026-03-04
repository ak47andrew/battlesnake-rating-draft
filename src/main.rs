use std::fs::File;
use csv::Writer;
use crate::battlesnake::Battlesnake;
use crate::game::simulate_game;
use crate::glecko::update_rating;
use crate::matchmaking::split_into_games;
use crate::stats::calculate;

mod battlesnake;
mod glecko;
mod game;
mod matchmaking;
mod stats;

const EPOCHS: usize = 10000;

fn main() {
    let mut iterations_log = Writer::from_writer(File::create(
        format!("metrics_{}.csv", EPOCHS.to_string())
    ).unwrap());

    iterations_log.write_record(&[
        "iteration",
        "spearman",
        "mean_rank_error",
        "top_k_overlap",
    ]).unwrap();

    let mut master_snakes: Vec<Battlesnake> = Vec::new();
    for _ in 0..50 {
        master_snakes.push(
            Battlesnake::new()
        )
    }

    for epoch in 0..EPOCHS {
        let games = split_into_games(master_snakes);

        let mut snakes: Vec<Battlesnake> = vec![];

        for game in games {
            let game = simulate_game(game);
            let mut game = update_rating(game);
            snakes.append(&mut game);
        }

        master_snakes = snakes;

        println!("Finished iteration #{}", epoch);
        let stats = calculate(&master_snakes);
        println!("Similarity: {:?}", stats);
        println!("===\n");
        iterations_log.write_record(&[
            epoch.to_string(),
            stats.spearman.to_string(),
            stats.mean_rank_error.to_string(),
            stats.top_k_overlap.to_string(),
        ]).unwrap();
    }

    iterations_log.flush().unwrap();
}
