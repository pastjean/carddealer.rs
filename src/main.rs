#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpServer};

mod deck;
mod webapp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state: web::Data<webapp::AppState> = web::Data::new(Default::default());

    let app = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/deck/", web::post().to(webapp::create_deck))
            .route("/deck/shuffle", web::post().to(webapp::shuffle_deck))
            .route("/deck/card", web::get().to(webapp::deal_card_from_deck))
    })
    .bind("0.0.0.0:8080")?
    .run();

    println!("Running on http://localhost:8080");

    app.await
}
