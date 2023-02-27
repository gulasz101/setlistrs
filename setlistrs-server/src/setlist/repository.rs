use anyhow::Result;
use setlistrs_types::{NewSetlist, Setlist, SetlistList, SetlistSong};
use sqlx::{query, Sqlite, SqlitePool, Transaction};

pub async fn find_all(pool: &SqlitePool) -> Result<SetlistList> {
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

pub async fn find_by_id(pool: &SqlitePool, setlist_id: i64) -> Result<Setlist> {
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

pub async fn create(pool: &SqlitePool, setlist: NewSetlist) -> Result<i64> {
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
        persist_setlist_song_relation(&mut transaction, &setlist_id, &song_id).await?;
    }

    transaction.commit().await?;

    Ok(setlist_id)
}

async fn persist_setlist_song_relation(
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

pub async fn delete(pool: &SqlitePool, setlist_id: i64) -> Result<()> {
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
