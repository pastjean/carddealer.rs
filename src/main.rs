extern crate actix;
extern crate actix_web;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::{http, middleware, server, App, HttpRequest, HttpResponse};
use std::sync::{Arc, RwLock};

pub mod deck;

use deck::{Card, Deck};

pub struct AppState {
    deck: Arc<RwLock<Option<Deck>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardResponse {
    suite: Option<String>,
    face: String,
}

impl From<Card> for CardResponse {
    fn from(c: Card) -> Self {
        CardResponse {
            face: format!("{:?}", c.0),
            suite: c.1.map(|suite| format!("{:?}", suite)),
        }
    }
}

pub fn create_deck(req: HttpRequest<AppState>) -> HttpResponse {
    let deck_arc = req.state().deck.clone();
    let mut deck = deck_arc.write().unwrap();
    match *deck {
        Some(_) => HttpResponse::with_body(http::StatusCode::BAD_REQUEST, "Deck already exists"),
        None => {
            *deck = Some(Deck::new());
            HttpResponse::Ok().body("Deck Created")
        }
    }
}

pub fn shuffle_deck(req: HttpRequest<AppState>) -> HttpResponse {
    let deck_arc = req.state().deck.clone();
    let mut deck = deck_arc.write().unwrap();
    match *deck {
        Some(ref mut deck) => {
            deck.shuffle();
            HttpResponse::Ok().body("Deck Shuffled")
        }
        None => HttpResponse::with_body(http::StatusCode::BAD_REQUEST, "Create your deck first"),
    }
}

pub fn deal_card_from_deck(req: HttpRequest<AppState>) -> HttpResponse {
    let deck_arc = req.state().deck.clone();
    let mut deck = deck_arc.write().unwrap();
    match *deck {
        Some(ref mut deck) => {
            let c = deck.deal_one_card();
            HttpResponse::Ok().json(c.map(|card| CardResponse::from(card)))
        }
        None => HttpResponse::with_body(http::StatusCode::BAD_REQUEST, "Create your deck first"),
    }
}

fn main() {
    let deck: Arc<RwLock<Option<Deck>>> = Arc::new(RwLock::new(None));

    let sys = actix::System::new("cards-dealer");

    server::new(move || {
        App::with_state(AppState{deck: deck.clone()}) // <- create app with state
                // enable logger
                .middleware(middleware::Logger::default())
                .route("/deck/", http::Method::POST, create_deck)
                .route("/deck/shuffle", http::Method::POST, shuffle_deck)
                .route("/deck/card", http::Method::GET, deal_card_from_deck)
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");

    let _ = sys.run();
}
