# game_rocket

game_rocket is part of a full-stack machine learning project designed to make use of 42,000 Steam game reviews on 5000 unique games.

Reviews were collected using the Steam partner API and SteamDB's top 5000 games (current as of 10/20/2022).

game_rocket is the backend portion of that full-stack application. It interacts with a PostgreSQL database filled with Steam review data using Diesel (Rust crate). The API will accept HTTP requests using the Rocket Rust crate.

## API

### /recommend/ endpoint
```
POST /recommend/
Accept: application/json
Content-Type: application/json
```

Examples:
list attributes you would like to be present in the recommended games in the "positives" list.
list attributes you would like not to be present in the recommended games in the "negatives" list.


Let's say you want 5 games where reviewers talked about the mechanics and found it interesting.
You don't want games people describe as easy / short.
You want games where the author recommended the game and whose review had a positive sentiment overall.
Here's what you would POST to the /recommend/ endpoint:
```
{
  "positives": ["interesting", "mechanics"],
  "negatives": ["easy", "short"],
  "weighted_vote_score": "true",
  "sentiment": "true",
  "n_games": 5
}
```

Let's say you want a recommendation for 1 horror game but you DO NOT want zombies to be in the game.
You don't even care if reviewers liked the game. You submit the following POST request to the /recommend/ endpoint:
```
{
  "positives": ["horror"],
  "negatives": ["zombies"],
  "weighted_vote_score": "false",
  "sentiment": "false",
  "n_games": 1
}
```

returns:
```
[
  {
    "score": 0.45
    "game_id": "123",
    "game_name": "F.E.A.R.",
    "review_text": "Bad game, story and gameplay but fun to mess around with if you have a friend to play co op"
  }
]
```

Note that the returned JSON will be a LIST (array) of entries, even if n_games == 1.

For example, if you had game_rocket hosted at 127.0.0.1:8000 you could use curl to make a post request using:
```
curl -X POST http://127.0.0.1:8000/recommend -H 'Content-Type: application/json' -d '{"n_reviews": 5, "positives": ["combat", "characters"], "negatives": ["fps"], "author_recommended_game": true, "sentiment": true}'
```
Which would return something like this:
```
[
{"score":0.2858920122320596,"game_name":"Sword and Fairy 7","game_id":"1543030","review_text":"Sword and fairy 7 is definitely one of the best RPGs I‚Äôve ever played the storyline and characters Are awesome the graphics are beautiful and the Combat is great and easy to master Plus having a female protagonist is a breath of fresh air as most RPGs have male protagonists Overall Sword and fairy 7 is a great RPG that deserves way more Attention and acclaim than it‚Äôs gotten It‚Äôs just as good as the final fantasy series I highly recommend it"},
{"score":0.2650506522429059,"game_name":"Total War: THREE KINGDOMS","game_id":"779340","review_text":"A very refreshing entry to the total war series, which embodies the spirit of Rome 1 and Medieval 2. Not the most re-playable or sophisticated total war entry, but none the less, a solid, well balanced game with gorgeous aesthetics, competent AI and possibly the best diplomacy system ever featured by total war."},{"score":0.26396618836065877,"game_name":"Reventure","game_id":"900270","review_text":"One of the best \"choose your story\" games that I played, the highly dynamic story and 100 wonderful endings and strong comedy make this a memorable experience."},
{"score":0.26284833306873134,"game_name":"Sector 724","game_id":"459410","review_text":"this is a little known game it seems, but it is a pretty solid affair - a good space tug of war style hex based game, kind of boardgame like but with an added space battle section - the space battle section is simplistic and doesnt have alot in the way of tactics but itis real time and does afford some options, though this is a game really of attrition and masses and whoever has the best and most ships wins really, a good number ship design and upgrade options and some other elements thrown in for in game choices too, - for 6.99 this game is a definite worthwhile buy"},
{"score":0.2597100491111021,"game_name":"Middle-earth‚Ñ¢: Shadow of Mordor‚Ñ¢","game_id":"241930","review_text":"Love it, gameplay is like batman like the combat and all, u can climb structures like in assassin's creed, I would recommend this game to anyone, even tho not a fan of LOTR, the Nemesis system is amazing, as enemies keeps growing powerful, changes the game."}
]
```

## Development Roadmap

### 1. Continuous, autonomic data updates
Currently, the steam review data is in a fixed state.
That is, the data represents reviews from the 5000 top games from SteamDB as of October 2022.
Although some games remain relevant, this state will neglect new game releases, new reviews of current games, etc.

Solution: Periodic scraping of new reviews based on changes in the SteamDB top 5000 list.

A program which checks SteamDB weekly for new games and scrapes top reviews from Steam would keep the data relevant and expand the recommendations of the platform.

Candidate approach(es): Time based trigger | Cron job, AWS Lambda, or Google Cloud Functions

### 2. Aggregate game data in recommendations
As of right now (11/24/22), the algorithm shows users reviews based solely on how well the contents of their search match the contents of reviews in the database (and optionally, the sentiment and rating the reviewer gave).
Many users will want, I think, to be able to consider a game's popularity on the whole when receiving recommendations, which is not currently possible.

Solution: Acquisition of aggregate data from Steam (per game)

Steam displays the % of players who would "recommend" the game after playing it on each game's store page.
Using this, we could easily add this metric to the recommendation algorithm.
Granted, some changes would need to be incorporated to both frontend, backend, and database of the application.

Frontend
- Addition of checkbox to include % recommended in the search
- Slight changes to the POST request upon Form submit.

Backend
- Addition of percent_recommended to the game_review Struct.
- Addition of include_percent_recommended flag to the Score function.
- Addition of include_percent_recommended flag in the accepted parameters in POST requests to the /recommend/ endpoint.

Database
- Addition of the percent_recommended column in the game_reviews table.


### Database backups / redundancy
Currently, all of the data used in Game Rocket is accessible from my local development environment.
Given either of the changes listed above, Game Rocket will begin relying on live, changing data.
In short, the velocity of the data will necessitate a change in approach.

Solution: Database Backups

Perhaps immediately following the program which collects new data, another program will create a backup of the database in long-term storage (AWS S3, GCP Storage, etc).
This would make the system more fault-tolerant and resilient.

Candidate approach(es): Time based trigger | AWS Lambda and AWS S3 or Google Cloud Functions and Google Cloud Storage
