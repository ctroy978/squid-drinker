table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
    }
}

table! {
    labels (id) {
        id -> Int4,
        ingredient_id -> Int4,
        label -> Varchar,
    }
}

table! {
    qtys (id) {
        id -> Int4,
        ingredient_id -> Int4,
        quantity -> Varchar,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
        rank -> Int4,
        booz -> Varchar,
        directions -> Varchar,
    }
}

table! {
    units (id) {
        id -> Int4,
        ingredient_id -> Int4,
        unit_description -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    ingredients,
    labels,
    qtys,
    recipes,
    units,
);
