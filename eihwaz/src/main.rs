use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use axum::{response::IntoResponse, Router, routing::get};
use axum::body::{Body, boxed};
use axum::http::{Response, StatusCode};
use clap::Parser;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::data::CardListData;

pub mod data;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
	/// set the log level
	#[clap(short = 'l', long = "log", default_value = "info")]
	log_level: String,

	/// set the listen addr
	#[clap(short = 'a', long = "addr", default_value = "localhost")]
	addr: String,

	/// set the listen port
	#[clap(short = 'p', long = "port", default_value = "8080")]
	port: u16,

	/// Set the directory where static files are to be found
	#[clap(long = "static-dir", default_value = "./dist")]
	static_dir: String,
}

#[tokio::main]
async fn main() {
	let opt = Opt::parse();

	// Setup logging & RUST_LOG from args
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
	}
	// Enable console logging
	tracing_subscriber::fmt::init();

	// Setup the app
	let app = Router::new()
		.route("/api/hello", get(hello))
		.route("/api/cards", get(get_cards))
		.route("/api/card/:id", get(get_card))
		.fallback_service(get(|req| async move {
			match ServeDir::new(&opt.static_dir).oneshot(req).await {
				Ok(res) => {
					let status = res.status();
					match status {
						StatusCode::NOT_FOUND => {
							let index_path = PathBuf::from(&opt.static_dir).join("index.html");
							let index_content = match fs::read_to_string(index_path).await {
								Ok(content) => content,
								Err(err) => {
									log::error!("Unable to read index.html: {}", err);
									return Response::builder()
										.status(StatusCode::INTERNAL_SERVER_ERROR)
										.body(boxed(Body::from("index not found")))
										.expect("Unable to build response");
								}
							};
							Response::builder()
								.status(StatusCode::OK)
								.body(boxed(Body::from(index_content)))
								.expect("Unable to build response")
						},
						_ => res.map(boxed),
					}
				},
				Err(err) => {
					log::error!("Unable to serve static files: {}", err);
					Response::builder()
						.status(StatusCode::INTERNAL_SERVER_ERROR)
						.body(boxed(Body::from(format!("error: {err}"))))
						.expect("Unable to build response")
				}
			}
		}))
		.layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

	let sock_addr = SocketAddr::from((
		IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
		opt.port,
	));

	log::info!("listening on http://{}", sock_addr);

	axum::Server::bind(&sock_addr)
		.serve(app.into_make_service())
		.await
		.expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
	"hello from server!"
}

async fn get_cards() -> impl IntoResponse {
	let cards = CardListData {
		cards: vec![
			data::CardData::new(1, "Card 1".to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam sapien neque, viverra ac augue sed, hendrerit tincidunt nunc. Etiam interdum mollis dolor. Cras vehicula dictum massa sit amet finibus. Duis id gravida urna, in ullamcorper libero. Mauris volutpat nisi id auctor tempor. Vivamus viverra nisi et sapien porttitor, nec auctor nisi pellentesque. Aliquam sed purus arcu. Ut eget ornare ex. Cras eu enim tellus. Aenean semper felis ac enim dictum mattis at quis risus. Curabitur vel leo a dolor tristique aliquet in in dolor.".to_string()),
			data::CardData::new(2, "Card 2".to_string(), "Description 2".to_string()),
			data::CardData::new(3, "Card 3".to_string(), "Description 3".to_string()),
			data::CardData::new(4, "Card 4".to_string(), "Description 4".to_string()),
			data::CardData::new(5, "Card 5".to_string(), "Description 5".to_string()),
			data::CardData::new(6, "Card 6".to_string(), "Description 6".to_string()),
			data::CardData::new(7, "Card 7".to_string(), "Description 7".to_string()),
			data::CardData::new(8, "Card 8".to_string(), "Description 8".to_string()),
			data::CardData::new(9, "Card 9".to_string(), "Description 9".to_string()),
			data::CardData::new(10, "Card 10".to_string(), "Description 10".to_string()),
		]
	};
	serde_json::to_string(&cards).expect("Unable to serialize card list")
}

async fn get_card(axum::extract::Path(id): axum::extract::Path<i32>) -> impl IntoResponse {
	let card = data::CardData::new(id, format!("Card {}", id), "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam sapien neque, viverra ac augue sed, hendrerit tincidunt nunc. Etiam interdum mollis dolor. Cras vehicula dictum massa sit amet finibus. Duis id gravida urna, in ullamcorper libero. Mauris volutpat nisi id auctor tempor. Vivamus viverra nisi et sapien porttitor, nec auctor nisi pellentesque. Aliquam sed purus arcu. Ut eget ornare ex. Cras eu enim tellus. Aenean semper felis ac enim dictum mattis at quis risus. Curabitur vel leo a dolor tristique aliquet in in dolor.".to_string());
	serde_json::to_string(&card).expect("Unable to serialize card")
}