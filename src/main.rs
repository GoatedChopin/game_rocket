use game_rocket::connect;
use diesel::prelude::*;
use models::*;
use scoring::*;
use game_rocket::*;


fn main() {
    use self::schema::game_reviews::dsl::*;

    let connection = &mut connect::establish_connection();
    let reviews = game_reviews
                            .load::<GameReview>(connection)
                            .expect("Error loading reviews");
    let tops = top_scores(reviews, 5, Some(vec![String::from("addictive"), String::from("tactical")]), None, true, true);
    println!("{:?}", tops)
}