use crate::battlesnake::Battlesnake;

const GAME_SIZE: usize = 4;

pub fn split_into_games(mut snakes: Vec<Battlesnake>) -> Vec<Vec<Battlesnake>> {
    let mut games: Vec<Vec<Battlesnake>> = vec![];

    while snakes.len() >= GAME_SIZE {
        snakes.sort_by_key(|x| x.last_played);
        let lead_snake = snakes.pop().unwrap();
        snakes.sort_by(|a, b|
            (a.rating.rating - lead_snake.rating.rating).abs().total_cmp(&(b.rating.rating - lead_snake.rating.rating).abs()));

        let mut game = vec![lead_snake];
        for _ in 0..GAME_SIZE - 1 {
            game.push(snakes.pop().unwrap());
        }
        games.push(game);
    }

    games
}