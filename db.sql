CREATE TABLE ingredients
(
    id           SERIAL       PRIMARY KEY,
    name              varchar(255) NOT NULL
);


CREATE TABLE recipes
(
    id          SERIAL          PRIMARY KEY,
    name            varchar(120)    NOT NULL,
    description     text            NOT NULL,
    DishSize        integer         NOT NULL
);

CREATE TABLE ingredientsToRecipes
(
    rec_id          integer         REFERENCES recipes (id),
    ing_id          integer         REFERENCES ingredients (id),
    quantity        integer         NOT NULL,
    quantity_unit   varchar(100)    NOT NULL,
    PRIMARY KEY(rec_Id, ing_id)
);