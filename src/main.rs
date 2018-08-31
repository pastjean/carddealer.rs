extern crate actix;
extern crate actix_web;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::server;
use std::sync::{Arc, RwLock};

pub mod deck;
pub mod webapp;

use deck::Deck;
use webapp::{app, AppState};

fn main() {
    let deck: Arc<RwLock<Option<Deck>>> = Arc::new(RwLock::new(None)).clone();

    let sys = actix::System::new("cards-dealer");

    server::new(move || app(AppState { deck: deck.clone() }))
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");

    sys.run();
}
