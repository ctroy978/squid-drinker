CREATE TABLE recipes(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    rank INT NOT NULL,
    directions VARCHAR NOT NULL
)