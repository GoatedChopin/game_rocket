use std::{collections::BinaryHeap, cmp::Ordering, str::FromStr};
use crate::models::GameReview;
use serde::Serialize;


pub fn top_scores(mut reviews: Vec<GameReview>, n_reviews: i32, positives: Option<Vec<String>>, negatives: Option<Vec<String>>, author_recommended_game: bool, sentiment: bool) -> Vec<ReviewPayload> {
    let mut i = 0;
    let mut bh = BinaryHeap::from([score_dex::new(-1.0, 0)]);
    let pos = positives.unwrap_or(vec![]);
    let neg = negatives.unwrap_or(vec![]);
    for review in &reviews {
        bh.push(score_dex::new(compute_sub_score(&review, Some(&pos), Some(&neg), author_recommended_game, sentiment), i));
        i += 1;
    }
    let mut top: Vec<ReviewPayload> = Vec::new();
    let mut s_d;
    let mut review;
    let mut payload;
    for index in 0..n_reviews {
        if bh.is_empty() {
            break;
        }
        s_d = bh.pop().unwrap();
        review = &mut reviews[s_d.index];
        payload = ReviewPayload::from_review(review, s_d.score);
        top.push(payload);
        // println!("{:?}", payload);

    }
    top
}

fn compute_sub_score(review: &GameReview, positives: Option<&Vec<String>>, negatives: Option<&Vec<String>>, author_recommended_game: bool, sentiment: bool) -> f64 {
    let mut score = 1.0;
    match positives {
        Some(_) => {
            for col in positives.unwrap() {
                // Multiply score by column val
                let field = review.get(String::from(col));
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
                let field = 1.0 - review.get(String::from(col));
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

#[derive(Debug, Serialize)]
pub struct ReviewPayload {
    score: f64,
    game_name: String,
    game_id: String,
    review_text: String
}

impl ReviewPayload {
    pub fn from_review(review: &GameReview, score: f64) -> ReviewPayload {
        ReviewPayload { 
            score: score, 
            game_name: review.game_name.as_ref().map(|s| s).unwrap().to_string(), 
            game_id: review.game_id.as_ref().map(|s| s).unwrap().to_string(), 
            review_text: review.review_text.as_ref().map(|s| s).unwrap().to_string()
        }
    }
}

#[derive(Debug)]
struct score_dex {
    score: f64,
    index: usize
}

impl score_dex {
    pub fn new(score: f64, index: usize) -> score_dex{
        score_dex {score, index}
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
