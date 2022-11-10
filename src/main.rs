use game_rocket::connect;
use diesel::prelude::*;
use models::*;
use game_rocket::*;


fn main() {
    use self::schema::game_reviews::dsl::*;

    let connection = &mut connect::establish_connection();
    let reviews = game_reviews
                            .filter(game_name.eq(String::from("Magazine Editor")))
                            .filter(author_recommended_game.eq(true))
                            .limit(5)
                            .load::<GameReview>(connection)
                            .expect("Error loading reviews");
    // println!("{:?}", reviews);
    for review in reviews {
        println!("{:?}", review);
        println!();
    }
}