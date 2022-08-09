CREATE TABLE units(
    id SERIAL PRIMARY KEY,
    ingredient_id INT NOT NULL,
    unit_description VARCHAR NOT NULL
)