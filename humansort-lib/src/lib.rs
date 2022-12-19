use std::{collections::HashSet, fmt::Display};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HumansortState {
    items: Vec<HumansortItem>,
    num_items: usize,
    current_idx: usize,
}

impl HumansortState {
    pub fn next(&self) -> Vec<String> {
        // Select the desired number of items with a preference for higher-rated
        // items. (This avoids prompting the user for more information on items
        // that they rated lower already.)
        let mut rng = rand::thread_rng();
        let mut indices = Vec::new();
        while indices.len() < self.num_items {
            let x = rng.gen_range(0_f32..1_f32);
            let y = if x <= 0.25 {
                x
            } else {
                (x - 0.1).powi(3) + 0.25
            };
            let y_rounded = (y * self.items.len() as f32).round() as usize;
            if !indices.contains(&y_rounded) {
                indices.push(y_rounded);
            }
        }
        let mut values = Vec::new();
        for idx in indices {
            values.push(self.items[idx].value.clone());
        }
        values
    }
    pub fn update(&mut self, new_data: &[String]) {
        // Assume that the first item is the "winner", and all others are the
        // "losers". Compute rating updates based on pairwise comparisons
        // between the first item and all the others.
        if new_data.len() < 2 {
            return;
        }

        let winner = &new_data[0];
        let losers = &new_data[1..];

        // Find the winner's rating.
        let winner_idx = self
            .items
            .iter()
            .enumerate()
            .find(|(_, l)| l.value == *winner)
            .unwrap()
            .0;
        let winner_rating = self.items[winner_idx].rating;
        let mut winner_rating_increase = 0.;
        for loser in losers.iter() {
            // Find the loser's rating.
            let loser_idx = self
                .items
                .iter()
                .enumerate()
                .find(|(_, l)| l.value == *loser)
                .unwrap()
                .0;
            let loser_rating = self.items[loser_idx].rating;

            // Compute the expected score of the winner in the match between the
            // winner and loser.
            let expected = 1. / (1. + 10_f32.powf(loser_rating - winner_rating));

            // Update the loser's rating.
            self.items[loser_idx].rating -= expected;

            // Add the winner's rating update to a running total.
            winner_rating_increase += expected;
        }
        // Update the winner's rating.
        self.items[winner_idx].rating += winner_rating_increase;
        // Sort descending by rating.
        self.items
            .sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
    }
}

impl From<Vec<String>> for HumansortState {
    fn from(strings: Vec<String>) -> Self {
        let mut state = Vec::new();
        // Ignore duplicate items.
        let mut unique = HashSet::new();
        for item in strings.iter() {
            if !unique.contains(item) {
                state.push(HumansortItem {
                    value: item.to_string(),
                    rating: 0.,
                });
                unique.insert(item.clone());
            }
        }
        HumansortState {
            items: state,
            num_items: 5,
            current_idx: 0,
        }
    }
}

impl Iterator for HumansortState {
    type Item = HumansortItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < self.items.len() {
            let next_item = self.items[self.current_idx].clone();
            self.current_idx += 1;
            Some(next_item)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HumansortItem {
    value: String,
    rating: f32,
}

impl Display for HumansortItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
