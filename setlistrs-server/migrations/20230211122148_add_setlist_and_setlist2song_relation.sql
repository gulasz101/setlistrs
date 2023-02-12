-- Add migration script here
CREATE TABLE setlists(
  id integer PRIMARY KEY, 
  display_title text
);

CREATE TABLE setlist_to_song_relations (
  id integer PRIMARY KEY, 
  setlist_id integer,
  song_id integer, 
  FOREIGN KEY(setlist_id) REFERENCES setlists(id),
  FOREIGN KEY(song_id) REFERENCES songs(id)
);
