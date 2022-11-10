import pandas as pd

df = pd.read_csv("game_reviews.csv")
df.drop_duplicates().to_csv("indexed_reviews.csv")