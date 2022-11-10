# game_rocket

game_rocket is part of a full-stack machine learning project designed to make use of 42,000 Steam game reviews on 5000 unique games.

Reviews were collected using the Steam partner API and SteamDB's top 5000 games (current as of 10/20/2022).

game_rocket is the backend portion of that full-stack application. It interacts with a PostgreSQL database filled with Steam review data using Diesel (Rust crate). The API will accept HTTP requests using the Rocket Rust crate.

## API (In development)

### /recommend/ endpoint
POST /recommend/
Accept: application/json
Content-Type: application/json

Examples:
list attributes you would like to be present in the recommended games in the "positive" list.
list attributes you would like not to be present in the recommended games in the "negative" list.


Let's say you want 5 games where reviewers talked about the mechanics and found it interesting.
You don't want games people describe as easy / short.
You want games where the author recommended the game and whose review had a positive sentiment overall.
Here's what you would POST to the /recommend/ endpoint:
```
{
  "positive": ["interesting", "mechanics"],
  "negative": ["easy", "short"],
  "weighted_vote_score": "true",
  "sentiment": "true",
  "n_games": 5
}
```

Let's say you want a recommendation for 1 horror game but you DO NOT want zombies to be in the game.
You don't even care if reviewers liked the game. You submit the following POST request to the /recommend/ endpoint:
```
{
  "positive": ["horror"],
  "negative": ["zombies"],
  "weighted_vote_score": "false",
  "sentiment": "false",
  "n_games": 1
}
```

returns:
```
{
  [
    "game_id": 123,
    "game_name": "F.E.A.R.",
    "review_text": "Bad game, story and gameplay but fun to mess around with if you have a friend to play co op"
  ]
}
```

Note that the returned JSON will be a LIST (array) of entries, even if n_games == 1.
