use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
	pub id: i32,
	pub name: String,
	pub description: String,
}

impl CardData {
	pub fn new(id: i32, name: String, description: String) -> Self {
		Self {
			id,
			name,
			description,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardListData {
	pub cards: Vec<CardData>,
}