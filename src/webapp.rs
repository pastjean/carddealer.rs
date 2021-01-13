use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};

use crate::deck::{Card, Deck};
use std::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    deck: RwLock<Option<Deck>>,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
struct DeckResponse {
    cards: Vec<CardResponse>,
}

impl From<Deck> for DeckResponse {
    fn from(d: Deck) -> Self {
        DeckResponse {
            cards: d
                .cards
                .clone()
                .iter()
                .map(|&f| CardResponse::from(f))
                .collect(),
        }
    }
}

pub async fn create_deck(data: web::Data<AppState>) -> HttpResponse {
    let mut deck = data.deck.write().unwrap();

    match *deck {
        Some(_) => HttpResponse::build(StatusCode::BAD_REQUEST).body("Deck already exists"),
        None => {
            let new_deck = Deck::default();
            *deck = Some(new_deck.clone());
            HttpResponse::Ok().json(DeckResponse::from(new_deck))
        }
    }
}

pub async fn shuffle_deck(data: web::Data<AppState>) -> HttpResponse {
    let mut deck = data.deck.write().unwrap();

    match *deck {
        Some(ref mut deck) => {
            deck.shuffle();
            HttpResponse::Ok().json(DeckResponse::from(deck.clone()))
        }
        None => HttpResponse::build(StatusCode::BAD_REQUEST).body("Create your deck first"),
    }
}

pub async fn deal_card_from_deck(data: web::Data<AppState>) -> HttpResponse {
    let mut deck = data.deck.write().unwrap();

    match *deck {
        Some(ref mut deck) => {
            let c = deck.deal_one_card();
            HttpResponse::Ok().json(c.map(|card| CardResponse::from(card)))
        }
        None => HttpResponse::build(StatusCode::BAD_REQUEST).body("Create your deck first"),
    }
}
