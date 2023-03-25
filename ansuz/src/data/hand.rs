use gloo_net::http::Request;
use yew::{html, Html};

use serde::{Deserialize, Serialize};
use stylist::css;

use crate::data::card::CardData;
use crate::data::Renderable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandData {
	cards: Vec<CardData>,
}

impl HandData {
	pub async fn from_server() -> Result<Self, String> {
		let resp = Request::get("/api/cards").send().await.unwrap();

		println!("HandData::from_server() resp: {:?}", resp);

		let result = {
			if !resp.ok() {
				Err(format!("Error fetching data: {} ({})", resp.status(), resp.status_text()))
			} else {
				resp.json::<HandData>().await.map_err(|err| format!("Error: {}", err))
			}
		};

		result
	}

	pub fn get_cards(&self) -> &Vec<CardData> {
		&self.cards
	}
}

impl Renderable for HandData {
	fn render(&self) -> Html {
		let half_card = CardData::card_em_width() / 2.0;

		html! {
			<div class={css!(r"
				position: fixed;
				bottom: -14em;
				width: 100%;
			")}>
			<ul class={css!(r"
				display: flex;
				flex-direction: row;
				align-items: center;
				justify-content: center;
				list-style: none;
				width: 100%;
				max-width: 50em;
				box-sizing: border-box;
				margin: 0 auto;

				transform: translateX(-${half_card}em);

				transition: all 0.5s ease;

				&:hover {
					transform: translateX(-${half_card}em) translateY(2em);
					max-width: 40em;
				}

			", half_card = half_card)}>{for self.cards.iter().enumerate().map(|(_i, card)| {
				html! {
					<li class={css!(r"
						overflow: visible;
						width: 1px;
						flex-grow: 1;

						transition: transform 0.5s ease;

						margin: 1em 0;

						z-index: 1;

						&:hover {
							transform: translateY(-15em);
						}
						")}>
						{ card.render() }
					</li>
				}
			})}</ul>
			</div>
		}
	}
}