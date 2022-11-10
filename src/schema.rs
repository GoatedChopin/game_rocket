// @generated automatically by Diesel CLI.

diesel::table! {
    game_reviews (pk) {
        pk -> Text,
        game_id -> Nullable<Text>,
        game_name -> Nullable<Text>,
        author_id -> Nullable<Text>,
        playtime -> Nullable<Int4>,
        weighted_vote_score -> Nullable<Float8>,
        author_recommended_game -> Nullable<Bool>,
        review_text -> Nullable<Text>,
        sentiment -> Nullable<Float8>,
        fun -> Nullable<Float8>,
        story -> Nullable<Float8>,
        gameplay -> Nullable<Float8>,
        graphics -> Nullable<Float8>,
        combat -> Nullable<Float8>,
        easy -> Nullable<Float8>,
        characters -> Nullable<Float8>,
        music -> Nullable<Float8>,
        world -> Nullable<Float8>,
        interesting -> Nullable<Float8>,
        simple -> Nullable<Float8>,
        short -> Nullable<Float8>,
        mechanics -> Nullable<Float8>,
        achievements -> Nullable<Float8>,
        difficulty -> Nullable<Float8>,
        puzzles -> Nullable<Float8>,
        friends -> Nullable<Float8>,
        fast -> Nullable<Float8>,
        original -> Nullable<Float8>,
        unique -> Nullable<Float8>,
        community -> Nullable<Float8>,
        space -> Nullable<Float8>,
        beautiful -> Nullable<Float8>,
        challenging -> Nullable<Float8>,
        strategy -> Nullable<Float8>,
        soundtrack -> Nullable<Float8>,
        fps -> Nullable<Float8>,
        funny -> Nullable<Float8>,
        horror -> Nullable<Float8>,
        dungeon -> Nullable<Float8>,
        shooter -> Nullable<Float8>,
        atmosphere -> Nullable<Float8>,
        crafting -> Nullable<Float8>,
        guns -> Nullable<Float8>,
        simulator -> Nullable<Float8>,
        upgrades -> Nullable<Float8>,
        zombies -> Nullable<Float8>,
        adventure -> Nullable<Float8>,
        casual -> Nullable<Float8>,
        monsters -> Nullable<Float8>,
        grinding -> Nullable<Float8>,
        satisfying -> Nullable<Float8>,
        magic -> Nullable<Float8>,
        deep -> Nullable<Float8>,
        sad -> Nullable<Float8>,
        platformer -> Nullable<Float8>,
        animation -> Nullable<Float8>,
        fantasy -> Nullable<Float8>,
        customization -> Nullable<Float8>,
        exploration -> Nullable<Float8>,
        addictive -> Nullable<Float8>,
        tactical -> Nullable<Float8>,
    }
}
