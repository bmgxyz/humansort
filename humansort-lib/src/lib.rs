use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HumansortState {
    state: Vec<HumansortItem>,
}

impl HumansortState {
    fn next(&self) -> Vec<&HumansortItem> {
        todo!()
    }
    fn update(&mut self, sorted: &[&HumansortItem]) {
        todo!()
    }
}

impl From<Vec<String>> for HumansortState {
    fn from(items: Vec<String>) -> Self {
        let mut state = Vec::new();
        // Ignore duplicate items.
        let mut unique = HashSet::new();
        for item in items.iter() {
            if !unique.contains(item) {
                state.push(HumansortItem {
                    value: item.to_string(),
                    rating: 0.,
                });
                unique.insert(item.clone());
            }
        }
        HumansortState { state }
    }
}

#[derive(Deserialize, Serialize)]
struct HumansortItem {
    value: String,
    rating: f32,
}
