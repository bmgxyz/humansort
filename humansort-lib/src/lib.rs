use std::{collections::HashSet, error::Error, fmt::Display};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct HumansortState {
    items: Vec<HumansortItem>,
    #[serde(skip, default = "default_num_items")]
    num_items: usize,
    #[serde(skip, default = "default_current_idx")]
    current_idx: usize,
}

fn default_num_items() -> usize {
    5
}

fn default_current_idx() -> usize {
    0
}

impl HumansortState {
    pub fn new() -> Self {
        HumansortState::default()
    }
    pub fn next(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // Select the desired number of items with a preference for higher-rated
        // items. (This avoids prompting the user for more information on items
        // that they rated lower already.)
        // TODO: return error if there aren't enough items
        let mut rng = rand::thread_rng();
        let mut indices = Vec::new();
        while indices.len() < self.num_items {
            let x = rng.gen_range(0_f32..1_f32);
            let y = if x <= 0.25 {
                x
            } else {
                (x - 0.1).powi(3) + 0.25
            };
            let y_rounded = (y * self.items.len() as f32).floor() as usize;
            if !indices.contains(&y_rounded) {
                indices.push(y_rounded);
            }
        }
        let mut values = Vec::new();
        for idx in indices {
            values.push(self.items[idx].value.clone());
        }
        Ok(values)
    }
    pub fn update(&mut self, new_data: &[String]) -> Result<(), Box<dyn Error>> {
        // Assume that the first item is the "winner", and all others are the
        // "losers". Compute rating updates based on pairwise comparisons
        // between the first item and all the others.
        if new_data.len() < 2 {
            return Err(format!("Must have at least two items, found {}", new_data.len()).into());
        }

        let winner = &new_data[0];
        let losers = &new_data[1..];

        // Find the winner's rating.
        let winner_idx = self.find_item_idx_by_value(winner)?;
        let winner_rating = self.items[winner_idx].rating;
        let mut winner_rating_increase = 0.;
        for loser in losers.iter() {
            // Find the loser's rating.
            let loser_idx = self.find_item_idx_by_value(loser)?;
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

        Ok(())
    }
    pub fn merge(&mut self, items_to_merge: &[String]) {
        let mut new_items = Vec::new();
        // Keep items that are in the new list. (This implicitly removes items
        // that are in the old list by not the new list.)
        for item in self.items.iter() {
            if items_to_merge.contains(&item.value) {
                new_items.push(item.clone());
            }
        }
        // Add items that are in the new list but not already in the old list.
        for item in items_to_merge {
            if !self.items.iter().any(|i| i.value == *item) {
                new_items.push(HumansortItem {
                    value: item.clone(),
                    rating: 0.,
                });
            }
        }
        self.items = new_items;
        // Sort descending by rating.
        self.items
            .sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
    }
    pub fn set_num_items(&mut self, new_num_items: usize) -> Result<(), Box<dyn Error>> {
        if new_num_items < 2 {
            return Err(format!(
                "Number of items to display must be >= 2 (got {})",
                new_num_items
            )
            .into());
        }
        self.num_items = new_num_items;
        Ok(())
    }
    pub fn num_items(&self) -> usize {
        self.num_items
    }
    pub fn get_all_items(&self) -> Vec<HumansortItem> {
        self.items.clone()
    }
    pub fn add_item(&mut self, new_item: &String) {
        self.items.push(HumansortItem {
            value: new_item.to_string(),
            ..Default::default()
        })
    }
    pub fn rename_item(
        &mut self,
        old_item_name: &String,
        new_item_name: &String,
    ) -> Result<(), Box<dyn Error>> {
        let item_idx = self.find_item_idx_by_value(old_item_name)?;
        self.items[item_idx].value = new_item_name.to_string();
        Ok(())
    }
    pub fn remove_item(&mut self, item_to_remove: &String) -> Result<(), Box<dyn Error>> {
        let item_idx = self.find_item_idx_by_value(item_to_remove)?;
        self.items.remove(item_idx);
        Ok(())
    }
    fn find_item_idx_by_value(&self, needle: &String) -> Result<usize, Box<dyn Error>> {
        let maybe_item = self
            .items
            .iter()
            .enumerate()
            .find(|(_, l)| l.value == *needle);
        if let Some(i) = maybe_item {
            Ok(i.0)
        } else {
            Err(format!("Failed to find '{}'", needle).into())
        }
    }
}

impl Default for HumansortState {
    fn default() -> Self {
        HumansortState {
            items: Vec::new(),
            num_items: 5,
            current_idx: 0,
        }
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct HumansortItem {
    value: String,
    rating: f32,
}

impl Display for HumansortItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for HumansortItem {
    fn default() -> Self {
        HumansortItem {
            value: String::new(),
            rating: 0.,
        }
    }
}
