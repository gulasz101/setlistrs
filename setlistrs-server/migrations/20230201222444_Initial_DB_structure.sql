CREATE TABLE songs (
  id integer primary key,
  name text,
  chords text 
);

CREATE TABLE links (
  id integer PRIMARY KEY, 
  display_title text null, 
  url text
);

CREATE TABLE sources (
  id integer PRIMARY KEY, 
  song_id integer, 
  link_id integer,
  FOREIGN KEY(song_id) REFERENCES songs(id),
  FOREIGN KEY(link_id) REFERENCES links(id)
);

CREATE TABLE covers(
  id integer PRIMARY KEY, 
  song_id integer, 
  link_id integer,
  FOREIGN KEY(song_id) REFERENCES songs(id),
  FOREIGN KEY(link_id) REFERENCES links(id)
);

