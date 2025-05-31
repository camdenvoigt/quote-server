CREATE TABLE IF NOT EXISTS quotes (
  quote_id SERIAL INT UNQIUE PRIMAY KEY,
  quote varchar(500),
  author VARCHAR(200)
);

CREATE TABLE IF NOT EXISTS tags_rel (
  quote_id INT NOT NULL,
  tag_id INT NOT NULL,
  FOREIGN KEY(quote_id) REFERENCES quotes(quote_id),
  FOREIGN KEY(tag_id) REFERENCES tags(tag_id)
);

CREATE TABLE IF NOT EXISTS tags (
  tag_id INT UNQIUE PRIMARY KEY,
  tag VARCHAR(100)
);
