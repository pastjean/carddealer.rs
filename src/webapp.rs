use actix_web::{http, middleware, App, HttpRequest, HttpResponse};
use std::sync::{Arc, RwLock};

use deck::{Card, Deck};

pub struct AppState {
    pub deck: Arc<RwLock<Option<Deck>>>,
}

pub fn new_deck() -> Arc<RwLock<Option<Deck>>> {
    Arc::new(RwLock::new(None)).clone()
}

pub fn app(initial_state: AppState) -> App<AppState> {
    App::with_state(initial_state) // <- create app with state
        // enable logger
        .middleware(middleware::Logger::default())
        .route("/deck/", http::Method::POST, create_deck)
        .route("/deck/shuffle", http::Method::POST, shuffle_deck)
        .route("/deck/card", http::Method::GET, deal_card_from_deck)
}

#[derive(Debug, Serialize, Deserialize)]
struct CardResponse {
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

fn create_deck(req: HttpRequest<AppState>) -> HttpResponse {
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

fn shuffle_deck(req: HttpRequest<AppState>) -> HttpResponse {
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

fn deal_card_from_deck(req: HttpRequest<AppState>) -> HttpResponse {
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
