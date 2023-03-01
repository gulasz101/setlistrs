use gloo_net::http::Request;

use setlistrs_types::Song;
use setlistrs_types::SongList;

use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::SongChockboxAction;
use crate::components::SongsForSetlist;

enum SonglistContentAction {
    PutSongs(SongList),
    RemoveSong(i64),
}

struct SonglistContent {
    pub songs: Vec<(i64, Song)>,
}

impl Reducible for SonglistContent {
    type Action = SonglistContentAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            SonglistContentAction::PutSongs(songlist) => SonglistContent::new(songlist).into(),
            SonglistContentAction::RemoveSong(song_id_to_remove) => {
                let mut songs = self.songs.clone();
                songs.retain(|(song_id, _song)| *song_id != song_id_to_remove);
                SonglistContent { songs }.into()
            }
        }
    }
}

impl SonglistContent {
    pub fn new(songlist: SongList) -> Self {
        Self {
            songs: songlist
                .data
                .into_iter()
                .map(|(song_id, song)| (song_id.clone(), song.clone()))
                .collect(),
        }
    }
}

#[function_component(SongsList)]
pub fn all_songs_list() -> Html {
    let setlist = use_reducer(|| SonglistContent { songs: vec![] });

    {
        let setlist = setlist.clone();
        use_effect_with_deps(
            move |_| {
                let setlist = setlist.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("http://127.0.0.1:8081/songs").send().await;
                    match response {
                        Ok(response) => {
                            let possible_setlist: Result<SongList, _> = response.json().await;
                            match possible_setlist {
                                Ok(s) => setlist.dispatch(SonglistContentAction::PutSongs(s)),
                                Err(_) => panic!(), // TODO -> handle it gracefuly
                            }
                        }
                        Err(_) => panic!(), // TODO -> handle it gracefuly
                    };
                });

                || ()
            },
            (),
        );
    }

    let songs_for_setlist = use_reducer(|| SongsForSetlist { data: Vec::new() });

    let on_song_checkbox_click = {
        let songs_for_setlist = songs_for_setlist.clone();
        Callback::from(move |event: yew::MouseEvent| {
            let checkbox: HtmlInputElement = event.target_unchecked_into();
            let song_id = checkbox
                .value()
                .parse::<i64>()
                .expect("Checkbox always has value.");
            if checkbox.checked() {
                songs_for_setlist.dispatch(SongChockboxAction::Add(song_id));
            } else {
                songs_for_setlist.dispatch(SongChockboxAction::Remove(song_id));
            }
        })
    };

    let on_song_delete_click = {
        let songs_for_setlist = songs_for_setlist.clone();
        let setlist = setlist.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let button: HtmlElement = event.target_unchecked_into();
            let song_id = match button.get_attribute("data-song-id") {
                Some(song_id) => song_id.parse::<i64>().unwrap(),
                None => panic!(),
            };

            let songs_for_setlist = songs_for_setlist.clone();
            let setlist = setlist.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response =
                    Request::delete(format!("http://127.0.0.1:8081/songs/{}", song_id).as_str())
                        .send()
                        .await;
                match response {
                    Ok(response) => {
                        if response.status() == 204 {
                            songs_for_setlist.dispatch(SongChockboxAction::Remove(song_id));
                            setlist.dispatch(SonglistContentAction::RemoveSong(song_id));
                        }
                    }
                    Err(_) => panic!(), // TODO -> handle it gracefuly
                };
            });
        })
    };

    html! {
    <>
        <article>
            <crate::components::SetlistAdd
                songs={(*songs_for_setlist).data.clone()}
                disabled={&(*songs_for_setlist).data.is_empty()}
            />
        </article>
        <table>
            <caption>{ "All songs" }</caption>
            <thead>
                <th>{ "Add to setlist" }</th>
                <th>{ "Song" }</th>
                <th>{ "Cover" }</th>
                <th>{ "Chords" }</th>
                <th>{ "Action" }</th>
            </thead>
            <tbody>
            {
                html! {
                    for (*setlist).songs.iter().map(|( song_id, song )|
                    html! {
                        <tr data-song_id={ format!("{}", song_id) }>
                            <td><input type="checkbox" name="song" onclick={ on_song_checkbox_click.clone() } value={ format!("{}", song_id) }/></td>
                            <td>{
                                html! {<>
                                    <p>{ song.name.clone() }</p>
                                    {for song.source.iter().map(|cover| html!
                                        {
                                            <li>
                                                <a href={ cover.url.clone() } target="_blank">{
                                                match cover.display_title.clone() {
                                                    Some(display_title) => display_title,
                                                    None => cover.url.clone(),
                                                }
                                                }</a>
                                            </li>
                                        })
                                    }
                                    </>
                                }

                            }</td>
                            <td>{
                                match &song.cover {
                                    Some(covers) => html! {
                                        for covers.iter().map(|cover| html!
                                        {
                                            <li>
                                                <a href={ cover.url.clone() } target="_blank">{
                                                match cover.display_title.clone() {
                                                    Some(display_title) => display_title,
                                                    None => cover.url.clone(),
                                                }
                                                }</a>
                                            </li>
                                        }
                                        )
                                    },
                                    None => "".into(),
                                }
                            }</td>
                            <td>{ song.chords.clone() }</td>
                            <td><button onclick={ on_song_delete_click.clone() } data-song-id={ format!("{}", song_id) } class={classes!("red-bg-bd")}>{"Delete"}</button></td>
                        </tr>
                    })
                }
            }
            </tbody>
        </table>
    </>
    }
}
