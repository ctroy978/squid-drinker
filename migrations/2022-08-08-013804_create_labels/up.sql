CREATE TABLE labels(
    id SERIAL PRIMARY KEY,
    ingredient_id INT NOT NULL,
    label VARCHAR NOT NULL
)