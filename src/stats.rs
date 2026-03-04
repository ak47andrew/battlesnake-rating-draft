use std::collections::{HashMap, HashSet};
use crate::battlesnake::Battlesnake;

#[derive(Debug)]
pub struct Stats {
    pub spearman: f64,
    pub mean_rank_error: f64,
    pub top_k_overlap: u32
}

pub fn calculate(players: &Vec<Battlesnake>) -> Stats {
    let mut by_skill: Vec<&Battlesnake> = players.iter().collect();
    let mut by_elo: Vec<&Battlesnake> = players.iter().collect();

    by_skill.sort_by(|a, b| a.skill.total_cmp(&b.skill));
    by_elo.sort_by(|a, b| a.rating.rating.total_cmp(&b.rating.rating));

    Stats {
        spearman: calculate_spearman(players, &by_skill, &by_elo),
        mean_rank_error: calculate_mean_abs_err(players, &by_skill, &by_elo),
        top_k_overlap: calculate_top_k_overlap(&by_skill, &by_elo),
    }
}

fn calculate_spearman(players: &Vec<Battlesnake>, by_skill: &Vec<&Battlesnake>, by_elo: &Vec<&Battlesnake>) -> f64 {
    let mut skill_rank = HashMap::new();
    for (i, p) in by_skill.iter().enumerate() {
        skill_rank.insert(p.id, i as f64);
    }

    let mut elo_rank = HashMap::new();
    for (i, p) in by_elo.iter().enumerate() {
        elo_rank.insert(p.id, i as f64);
    }

    let n = players.len() as f64;
    let mut sum_d2 = 0.0;

    for p in players {
        let d = skill_rank[&p.id] - elo_rank[&p.id];
        sum_d2 += d * d;
    }

    1.0 - (6.0 * sum_d2) / (n * (n*n - 1.0))
}

fn calculate_mean_abs_err(players: &Vec<Battlesnake>, by_skill: &Vec<&Battlesnake>, by_elo: &Vec<&Battlesnake>) -> f64 {
    let mut skill_rank = HashMap::new();
    for (i, p) in by_skill.iter().enumerate() {
        skill_rank.insert(p.id, i as f64);
    }

    let mut elo_rank = HashMap::new();
    for (i, p) in by_elo.iter().enumerate() {
        elo_rank.insert(p.id, i as f64);
    }

    let mut error = 0.0;

    for p in players {
        error += (skill_rank[&p.id] - elo_rank[&p.id]).abs();
    }

    error / players.len() as f64
}

fn calculate_top_k_overlap(by_skill: &Vec<&Battlesnake>, by_elo: &Vec<&Battlesnake>) -> u32 {
    let k = 10;

    let top_skill: HashSet<_> = by_skill.iter().take(k).map(|x| x.id).collect();
    let top_elo: HashSet<_> = by_elo.iter().take(k).map(|x| x.id).collect();

    top_skill.intersection(&top_elo).count() as u32
}
