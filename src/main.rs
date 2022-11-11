use game_rocket::connect;
use diesel::prelude::*;
use models::*;
use rocket::{launch, build, post, routes};
use rocket::serde::{Deserialize, json::{Json, json}};
use scoring::*;
use game_rocket::*;
use crate::schema::game_reviews::dsl::game_reviews;


#[launch]
fn rocket() -> _ {
    // use self::schema::game_reviews::dsl::*;

    // let connection = &mut connect::establish_connection();
    // let reviews = game_reviews
    //                         .load::<GameReview>(connection)
    //                         .expect("Error loading reviews");
    // let tops = top_scores(reviews, 5, Some(vec![String::from("addictive"), String::from("tactical")]), None, true, true);
    // println!("{:?}", tops);
    rocket::build().mount("/recommend", routes![recommend])
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