use anyhow::Result;
use setlistrs_types::Song;
use setlistrs_types::YTLink;
use sqlx::Sqlite;
use sqlx::Transaction;
use sqlx::{query, SqlitePool};

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
