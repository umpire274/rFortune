// src/fortune.rs
use rand::rng;
use rand::seq::IndexedRandom;

pub fn get_random_fortune(fortunes: &[String]) -> Option<String> {
    let mut rng = rng();
    fortunes.choose(&mut rng).cloned()
}
