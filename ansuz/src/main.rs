use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::data::card::CardData;
use crate::data::hand::HandData;
use crate::data::Renderable;

pub mod data;

#[derive(Clone, Routable, PartialEq)]
enum Route {
	#[at("/")]
	Home,

	#[at("/hello-server")]
	HelloServer,

	#[at("/card/:id")]
	Card { id: i32 },

	#[at("/cards")]
	Cards,

	#[not_found]
	#[at("/404")]
	NotFound,
}

fn switch(routes: Route) -> Html {
	match routes {
		Route::Home => html! {
			<div>
				<h1>{ "Hello Frontend" }</h1>
				<Link<Route> to={Route::Cards}>{ "See the cards" }</Link<Route>>
			</div>
		},
		Route::HelloServer => html! { <HelloServer /> },
		Route::Cards => html! { <CardList /> },
		Route::Card { id } => html! { <Card id={id} /> },
		Route::NotFound => html! { <h1>{ "404: Not Found" }</h1> },
	}
}

#[function_component(App)]
fn app() -> Html {
	html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
	wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
	console_error_panic_hook::set_once();
	yew::Renderer::<App>::new().render();
}

#[function_component(HelloServer)]
fn hello_server() -> Html {
	let data = use_state(|| None);

	{
		let data = data.clone();
		use_effect(move || {
			if data.is_none() {
				spawn_local(async move {
					let resp = Request::get("/api/hello").send().await.unwrap();
					let result = {
						if !resp.ok() {
							Err(format!("Error fetching data: {} ({})", resp.status(), resp.status_text()))
						} else {
							resp.text().await.map_err(|err| format!("Error: {}", err))
						}
					};
					data.set(Some(result));
				});
			}
			|| ()
		})
	}

	match data.as_ref() {
		None => html! { <div>{ "No server response" }</div> },
		Some(Ok(data)) => html! { <div>{ data }</div> },
		Some(Err(err)) => html! { <div>{ err }</div> },
	}
}

#[derive(Clone, Properties, PartialEq)]
pub struct CardProps {
	pub id: i32,
}

#[function_component(Card)]
fn card(props: &CardProps) -> Html {
	let data = use_state(|| None);

	let id = props.id;

	{
		let data = data.clone();
		use_effect(move || {
			if data.is_none() {
				spawn_local(async move {
					data.set(Some(CardData::from_server(id).await));
				});
			}
			|| ()
		})
	}

	match data.as_ref() {
		None => CardData::default().render(),
		Some(Ok(card)) => card.render(),
		Some(Err(err)) => html! { <div>{ err }</div> },
	}
}

#[function_component(CardList)]
fn card_list() -> Html {
	let data = use_state(|| None);

	{
		let data = data.clone();
		use_effect(move || {
			if data.is_none() {
				spawn_local(async move {
					data.set(Some(HandData::from_server().await));
				});
			}
			|| ()
		})
	}

	match data.as_ref() {
		None => html! { <div>{ "No server response" }</div> },
		Some(Ok(hand)) => html! {
			<div>
				<h1>{ "Cards" }</h1>
				{hand.render()}
			</div>
		},
		Some(Err(err)) => html! { <div>{ err }</div> },
	}
}