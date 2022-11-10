
lines = []
i = 0
with open("indexed_game_reviews.sql", "r") as file:
    for line in file:
        lines.append(line)
        if line.__contains__(";"):
            i += 1
            lines.append("\nCOMMIT;")

print("Replaced {} lines with commits".format(i))

with open("commit_indexed_game_reviews.sql", "w") as file:
    file.writelines(lines)