CREATE TABLE IF NOT EXISTS species(
  id SERIAL,
  name text,
  classification text,
  designation text,
  average_height text,
  skin_colors text,
  hair_colors text,
  eye_colors text,
  average_lifespan text,
  language text,
  homeworld text,
  PRIMARY KEY (id)
);

TRUNCATE TABLE species RESTART IDENTITY CASCADE;

COPY species(name, classification, designation, average_height, skin_colors, hair_colors, eye_colors, average_lifespan, language, homeworld)
FROM '/var/lib/postgresql/csv/species.csv'
DELIMITER ','
CSV HEADER;