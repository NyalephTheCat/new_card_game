use serde::{Deserialize, Serialize};

use crate::data::card::CardData;
use crate::data::hand::HandData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentData {
	head: Option<CardData>,
	torso: Option<CardData>,
	legs: Option<CardData>,
	necklace: Option<CardData>,
	left_hand: Option<CardData>,
	right_hand: Option<CardData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
	id: i32,
	name: String,
	nb_cards: i32,

	equipment: EquipmentData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
	your_hand: HandData,

	players: Vec<PlayerData>,
}