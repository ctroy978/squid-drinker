CREATE TABLE recipes(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL UNIQUE,
    rank INT NOT NULL,
    booz VARCHAR NOT NULL,
    directions VARCHAR NOT NULL
)