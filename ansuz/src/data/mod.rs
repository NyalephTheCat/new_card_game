use yew::prelude::*;

pub mod card;
pub mod hand;
pub mod table;

pub trait Renderable {
	fn render(&self) -> Html;
}