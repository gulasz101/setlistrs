use anyhow::Result;
use chrono::Utc;
use setlistrs_types::NewSetlist;
use setlistrs_types::Setlist;
use setlistrs_types::SetlistList;
use setlistrs_types::SetlistSong;
use setlistrs_types::Song;
use setlistrs_types::YTLink;
use sqlx::query;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;

pub async fn list_all_songs(pool: &SqlitePool) -> Result<Vec<(i64, Song)>> {
    let songs: Vec<(i64, Song)> = query!(
        r#"
SELECT id, name, chords 
FROM songs
WHERE deleted_at IS NULL
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|song| {
        (
            song.id,
            Song {
                name: match song.name {
                    Some(name) => name.into(),
                    None => "".into(),
                },
                source: Vec::new(),
                chords: match song.chords {
                    Some(chords) => chords.into(),
                    None => "".into(),
                },
                cover: None,
            },
        )
    })
    .collect();

    let song_ids = songs.iter().map(|(song_id, _song)| song_id);
    let mut covers: Vec<(i64, Vec<YTLink>)> = Vec::new();
    for song_id in song_ids.clone() {
        covers.push((song_id.clone(), obtain_covers(pool, song_id).await?));
    }

    let mut sources: Vec<(i64, Vec<YTLink>)> = Vec::new();
    for song_id in song_ids {
        sources.push((song_id.clone(), obtain_sources(pool, song_id).await?));
    }

    let songs_with_relations: Vec<(i64, Song)> = songs
        .into_iter()
        .map(|(song_id, song)| {
            let song_sources: Vec<YTLink> = match sources
                .iter()
                .find(|(source_song_id, _yt_link)| *source_song_id == song_id)
                .map(|(_source_song_id, yt_link)| yt_link)
                .cloned()
            {
                Some(links) => links,
                None => panic!(),
            };

            let song_covers: Vec<YTLink> = match covers
                .iter()
                .find(|(cover_song_id, _yt_link)| *cover_song_id == song_id)
                .map(|(_cover_song_id, yt_link)| yt_link)
                .cloned()
            {
                Some(links) => links,
                None => panic!(),
            };

            (
                song_id,
                Song {
                    name: song.name,
                    chords: song.chords,
                    source: song_sources,
                    cover: Some(song_covers),
                },
            )
        })
        .collect();

    Ok(songs_with_relations)
}

async fn obtain_covers(pool: &SqlitePool, song_id: &i64) -> Result<Vec<YTLink>> {
    Ok(query!(
        r#"
SELECT l.url, l.display_title FROM covers c, links l 
WHERE c.song_id = ? 
AND l.id = c.link_id
ORDER BY c.id
            "#,
        song_id
    )
    .map(|link| YTLink {
        url: match link.url {
            Some(url) => url.into(),
            None => "".into(),
        },
        display_title: link.display_title,
    })
    .fetch_all(pool)
    .await?)
}

async fn obtain_sources(pool: &SqlitePool, song_id: &i64) -> Result<Vec<YTLink>> {
    Ok(query!(
        r#"
SELECT l.url, l.display_title FROM sources s, links l 
WHERE s.song_id = ? 
AND l.id = s.link_id
ORDER BY s.id
            "#,
        song_id
    )
    .map(|link| YTLink {
        url: match link.url {
            Some(url) => url.into(),
            None => "".into(),
        },
        display_title: link.display_title,
    })
    .fetch_all(pool)
    .await?)
}

pub async fn persist_song(pool: &SqlitePool, song: Song) -> Result<Song> {
    let mut transaction = pool.begin().await?;

    let song_id = query!(
        r#"
        INSERT INTO songs(name, chords)
        VALUES(?, ?)
        "#,
        song.name,
        song.chords
    )
    .execute(&mut transaction)
    .await?
    .last_insert_rowid();

    let mut source_link_ids = Vec::with_capacity(song.source.len());
    for yt_link in &song.source {
        source_link_ids.push(persist_link(&mut transaction, &yt_link).await?);
    }

    let cover_link_ids = match &song.cover {
        Some(yt_links) => {
            let mut cover_link_ids = Vec::new();
            for yt_link in yt_links {
                cover_link_ids.push(persist_link(&mut transaction, &yt_link).await?);
            }

            cover_link_ids
        }
        None => todo!(),
    };
    for source_link_id in source_link_ids {
        persist_song_link_relation(
            &mut transaction,
            LinkRelationType::Source,
            song_id,
            source_link_id,
        )
        .await?;
    }
    for cover_link_id in cover_link_ids {
        persist_song_link_relation(
            &mut transaction,
            LinkRelationType::Cover,
            song_id,
            cover_link_id,
        )
        .await?;
    }

    transaction.commit().await?;

    Ok(song)
}

async fn persist_link(transaction: &mut Transaction<'_, Sqlite>, yt_link: &YTLink) -> Result<i64> {
    Ok(query!(
        r#"
                INSERT INTO links(display_title, url)
                VALUES (?, ?)
                "#,
        yt_link.display_title,
        yt_link.url,
    )
    .execute(transaction)
    .await?
    .last_insert_rowid())
}

enum LinkRelationType {
    Cover,
    Source,
}

async fn persist_song_link_relation(
    transaction: &mut Transaction<'_, Sqlite>,
    link_type: LinkRelationType,
    song_id: i64,
    link_id: i64,
) -> Result<i64> {
    Ok(match link_type {
        LinkRelationType::Cover => query!(
            r#"
            INSERT INTO covers(song_id, link_id)
            VALUES(?, ?)
               "#,
            song_id,
            link_id
        ),
        LinkRelationType::Source => query!(
            r#"
            INSERT INTO sources(song_id, link_id)
            VALUES(?, ?)
               "#,
            song_id,
            link_id
        ),
    }
    .execute(transaction)
    .await?
    .last_insert_rowid())
}

pub async fn mark_song_as_deleted(pool: &SqlitePool, song_id: i64) -> Result<i64> {
    let timestamp = Utc::now().timestamp();
    Ok(query!(
        r#"
UPDATE songs SET deleted_at = ? WHERE id = ?
        "#,
        timestamp,
        song_id,
    )
    .execute(pool)
    .await?
    .rows_affected() as i64)
}

pub async fn obtain_setlist_by_id(pool: &SqlitePool, setlist_id: i64) -> Result<Setlist> {
    let setlist_display_title: String = query!(
        r#"
SELECT display_title
FROM setlists
WHERE id = ?
        "#,
        setlist_id
    )
    .map(|setlist| match setlist.display_title {
        Some(display_title) => display_title,
        None => panic!(),
    })
    .fetch_one(pool)
    .await?;

    let songs: Vec<(i64, SetlistSong)> = match query!(
        r#"
SELECT s.id, s.name, s.chords
FROM songs s, setlist_to_song_relations stsr
WHERE stsr.setlist_id = ?
AND s.id = stsr.song_id
                  "#,
        setlist_id
    )
    .map(|song| {
        (
            song.id,
            SetlistSong {
                display_title: song.name.unwrap(),
                chords: song.chords.unwrap(),
            },
        )
    })
    .fetch_all(pool)
    .await
    {
        Ok(songs) => songs,
        Err(_) => Vec::new(),
    };

    Ok(Setlist {
        display_title: setlist_display_title,
        songs,
    })
}

pub async fn persist_setlist(pool: &SqlitePool, setlist: NewSetlist) -> Result<i64> {
    let mut transaction = pool.begin().await?;
    let setlist_id = query!(
        r#"
                INSERT INTO setlists(display_title)
                VALUES (?)
                "#,
        setlist.display_title,
    )
    .execute(&mut transaction)
    .await?
    .last_insert_rowid();

    for song_id in setlist.songs {
        perist_setlist_song_relation(&mut transaction, &setlist_id, &song_id).await?;
    }

    transaction.commit().await?;

    Ok(setlist_id)
}

async fn perist_setlist_song_relation(
    transaction: &mut Transaction<'_, Sqlite>,
    setlist_id: &i64,
    song_id: &i64,
) -> Result<i64> {
    Ok(query!(
        r#"
INSERT INTO setlist_to_song_relations(setlist_id, song_id)
VALUES(?, ?)
        "#,
        setlist_id,
        song_id,
    )
    .execute(transaction)
    .await?
    .last_insert_rowid())
}

pub async fn obtain_setlists_list(pool: &SqlitePool) -> Result<SetlistList> {
    Ok(SetlistList {
        data: query!(
            r#"
SELECT s.id, s.display_title FROM setlists s
ORDER BY s.id
            "#,
        )
        .map(|setlist| {
            (
                setlist.id,
                match setlist.display_title {
                    Some(display_title) => display_title,
                    None => panic!(),
                },
            )
        })
        .fetch_all(pool)
        .await?,
    })
}

pub async fn delete_setlist_by_id(pool: &SqlitePool, setlist_id: i64) -> Result<()> {
    let mut transaction = pool.begin().await?;
    query!(
        r#"
DELETE FROM setlist_to_song_relations
WHERE setlist_id = ?
        "#,
        setlist_id
    )
    .execute(&mut transaction)
    .await?;

    query!(
        r#"
DELETE FROM setlists
WHERE id = ?
        "#,
        setlist_id
    )
    .execute(&mut transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
