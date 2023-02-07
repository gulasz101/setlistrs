use anyhow::Result;
use setlistrs_types::Song;
use setlistrs_types::YTLink;
use sqlx::query;
use sqlx::query_as;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;

pub async fn list_all_songs(pool: &SqlitePool) -> Result<Vec<(i64, Song)>> {
    let songs: Vec<(i64, Song)> = query!(
        r#"
SELECT id, name, chords 
FROM songs
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
    let mut covers: Vec<(i64, YTLink)> = Vec::new();
    for song_id in song_ids.clone() {
        covers.push((song_id.clone(), obtain_cover(pool, song_id).await?));
    }

    let mut sources: Vec<(i64, Vec<YTLink>)> = Vec::new();
    for song_id in song_ids {
        sources.push((song_id.clone(), obtain_sources(pool, song_id).await?));
    }

    // let test: Vec<Vec<YTLink>> = sources
    //     .into_iter()
    //     .map(|(song_id, cover)| cover.clone())
    //     .collect();

    songs.iter().map(|(song_id, song)| {
        let song_sources: Vec<Vec<YTLink>> = sources
            .iter()
            .filter_map(|(source_song_id, yt_link)| match source_song_id {
                song_id => Some(yt_link),
                _ => None,
            })
            .cloned()
            .collect();

        song
    });

    Ok(songs)
}

async fn obtain_cover(pool: &SqlitePool, song_id: &i64) -> Result<YTLink> {
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
    .fetch_one(pool)
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
