use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use game_rocket::connect;
use diesel::prelude::*;
use models::*;
use game_rocket::*;


fn main() {
    use self::schema::game_reviews::dsl::*;

    let connection = &mut connect::establish_connection();
    let reviews = game_reviews
                            // .filter(game_name.eq(String::from("Magazine Editor")))
                            // .filter(author_recommended_game.eq(true))
                            // .limit(10)
                            .load::<GameReview>(connection)
                            .expect("Error loading reviews");
    // for review in reviews {
    //     println!("{:?}", review);
    //     println!("{}", compute_sub_score(&review, Some(vec![String::from("addictive"), String::from("tactical")]), None, true, true));
    //     println!();
    // }
    let mut i = 0;
    let mut bh = BinaryHeap::from([score_dex::new(-1.0, -1)]);
    for review in reviews {
        bh.push(score_dex::new(compute_sub_score(&review, Some(vec![String::from("addictive"), String::from("tactical")]), None, true, true), i));
        i += 1;
    }
    for index in 0..5 {
        println!("{:?}", bh.pop())
    }
}

#[derive(Debug)]
struct score_dex {
    score: f64,
    index: i32
}

impl score_dex {
    pub fn new(score: f64, index: i32) -> score_dex{
        score_dex {score: score, index: index}
    }
}

impl PartialEq for score_dex {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for score_dex {}

impl PartialOrd for score_dex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score > other.score {
            Some(Ordering::Greater)
        } else if self.score < other.score {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for score_dex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score > other.score {
            Ordering::Greater
        } else if self.score < other.score {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

fn compute_sub_score(review: &GameReview, positives: Option<Vec<String>>, negatives: Option<Vec<String>>, author_recommended_game: bool, sentiment: bool) -> f64 {
    let mut score = 1.0;
    match positives {
        Some(_) => {
            for col in positives.unwrap() {
                // Multiply score by column val
                let field = review.get(col);
                // println!("{}", field);
                score *= field;
            }
        },
        None => {
            // println!("No positives");
        }
    };
    match negatives {
        Some(_) => {
            for col in negatives.unwrap() {
                // Multiply score by column val
                let field = 1.0 - review.get(col);
                score *= field;
            }
        },
        None => {
            // print!("No negatives");
        }
    };
    match author_recommended_game {
        true => score *= review.get(String::from_str("author_recommended_game").unwrap()),
        false => {}
    };
    match sentiment {
        true => score *= review.get(String::from_str("sentiment").unwrap()),
        false => {}
    };
    score
}