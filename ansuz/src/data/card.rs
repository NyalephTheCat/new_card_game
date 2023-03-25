use gloo_net::http::Request;
use yew::prelude::*;

use serde::{Deserialize, Serialize};
use stylist::css;

use crate::data::Renderable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
	id: i32,
	name: String,
	description: String,
}

impl CardData {
	pub async fn from_server(id: i32) -> Result<Self, String> {
		let resp = Request::get(&format!("/api/card/{}", id)).send().await.unwrap();

		if !resp.ok() {
			Err(format!("Error fetching data: {} ({})", resp.status(), resp.status_text()))
		} else {
			resp.json::<CardData>().await.map_err(|err| format!("Error: {}", err))
		}
	}

	pub fn get_id(&self) -> i32 {
		self.id
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_description(&self) -> &String {
		&self.description
	}

	pub fn card_em_width() -> f32 {
		10.0
	}
}

impl Default for CardData {
	fn default() -> Self {
		Self {
			id: -1,
			name: "Loading...".to_string(),
			description: "This is a template for a loading card".to_string(),
		}
	}
}

impl Renderable for CardData {
	fn render(&self) -> Html {
		html! {
			<div class={css!(r"
				display: inline-block;
				width: ${card_em_width}em;
				height: 14em;
				border: 1px solid #666;
				border-radius: .3em;
				padding: .25em;
				margin: 0 .5em .5em 0;
				font-size: 1.2em;
				font-weight: normal;
				font-family: Arial, sans-serif;
				position: relative;
				box-shadow: .2em .2em .5em #333;
				user-select: none;

				background-color: #E7D7C1;

				&:hover {
					box-shadow: .2em .2em .5em #000;
				}
			", card_em_width = CardData::card_em_width())}>
				<div class={css!(r"
					border-bottom: 1px solid #ccc;
					margin-bottom: 0.5rem;
					color: #BF4342;

					font-family: 'Josefin Sans', sans-serif;
					font-weight: 700;
					font-size: 1em;
				")}>
					{ &self.name }
				</div>
				<div class={css!(r"
					position: absolute;
					height: 9em;
					bottom: 1.2em;
					font-family: 'Josefin Sans', sans-serif;
					font-weight: 400;
					font-size: 0.8em;
					color: #BF4342;
					overflow: hidden;
					/* make it scrollable */
					overflow-y: scroll;
				")}>
					{ &self.description }
				</div>
				<div class={css!(r"
					position: absolute;
					bottom: 0;
					left: 5px;
					color: #BF4342;

					font-family: 'Josefin Sans', sans-serif;
					font-weight: 700;
					font-size: 0.7em;
				")}>
					{"#"}{ &self.id }
				</div>
			</div>
		}
	}
}