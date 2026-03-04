use std::sync::atomic::{AtomicUsize, Ordering};
use chrono::{DateTime, Utc};
use rand::random;
use skillratings::glicko2::Glicko2Rating;

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
pub struct Battlesnake {
    pub id: usize,
    pub rating: Glicko2Rating,
    pub last_played: DateTime<Utc>,
    pub skill: f64,
}
impl Battlesnake {
    pub fn new() -> Self {
        Battlesnake {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            rating: Glicko2Rating::new(),
            last_played: Utc::now(),
            skill: random()
        }
    }
}