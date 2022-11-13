use game_rocket::connect;
use diesel::prelude::*;
use models::*;
use rocket::{launch, post, routes};
use rocket::serde::{Deserialize, json::Json};
use scoring::*;
use game_rocket::*;
use crate::schema::game_reviews::dsl::game_reviews;
use rocket::http::{Header, Method, ContentType, Status};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        if _request.method() == Method::Options {
            let body = "";
            response.set_header(ContentType::Plain);
            response.set_sized_body(body.len(), std::io::Cursor::new(body));
            response.set_status(Status::Ok);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/recommend", routes![recommend]).attach(CORS)
}

#[post("/", format="json", data="<game_request>")]
fn recommend(game_request: Json<GameRequest>) -> Json<Vec<ReviewPayload>> {
    let reviews = game_reviews.load::<GameReview>(&mut connect::establish_connection()).expect("Error loading reviews");
    let tops = top_scores(reviews, 
        game_request.n_reviews, 
        Some(game_request.positives.as_ref().map(|v| v).unwrap().to_owned()), 
        Some(game_request.negatives.as_ref().map(|v| v).unwrap().to_owned()), 
        game_request.author_recommended_game, 
        game_request.sentiment); 
    Json::from(tops)
}

#[derive(Deserialize)]
struct GameRequest {
    n_reviews: i32,
    positives: Option<Vec<String>>, 
    negatives: Option<Vec<String>>, 
    author_recommended_game: bool, 
    sentiment: bool
}