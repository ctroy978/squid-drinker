CREATE TABLE qtys(
    id SERIAL PRIMARY KEY,
    ingredient_id INT NOT NULL,
    quantity VARCHAR NOT NULL
)