extern crate actix;
extern crate actix_web;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::server;

pub mod deck;
pub mod webapp;

use webapp::{app, AppState, new_deck};

fn main() {
    let deck_state = new_deck();

    let sys = actix::System::new("cards-dealer");

    server::new(move || app(AppState { deck: deck_state.clone() }))
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");

    sys.run();
}
