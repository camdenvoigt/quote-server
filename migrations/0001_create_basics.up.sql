CREATE TABLE IF NOT EXISTS quotes (
  quote_id SERIAL INT UNQIUE NOT NULL,
  quote varchar(500) NOT NULL,
  author VARCHAR(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS tags_rel (
  quote_id INT NOT NULL,
  tag_id INT NOT NULL,
  FOREIGN KEY(quote_id) REFERENCES quotes(quote_id),
  FOREIGN KEY(tag_id) REFERENCES tags(tag_id)
);

CREATE TABLE IF NOT EXISTS tags (
  tag_id INT UNQIUE NOT NULL,
  tag VARCHAR(100) NOT NULL
);
