use diesel::prelude::*;
use self::models::*;
use game_rocket::*;
use connect;


pub fn main() {
    use self::schema::game_reviews::dsl::*;

    let connection = &mut connect::establish_connection();
    let reviews = game_reviews.limit(5).load::<GameReview>(connection).expect("Error loading reviews");
    reviews
    // let results = posts
    //     .filter(published.eq(true))
    //     .limit(5)
    //     .load::<Post>(connection)
    //     .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("-----------\n");
    //     println!("{}", post.body);
    // }
}