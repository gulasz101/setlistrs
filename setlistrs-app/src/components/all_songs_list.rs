use gloo_net::http::Request;
use setlistrs_types::NewSetlist;
use setlistrs_types::SongList;
use web_sys::{FormData, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[function_component(AllSongsList)]
pub fn all_songs_list() -> Html {
    let setlist = use_state(|| None);

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
                                Ok(s) => setlist.set(Some(s)),
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

    enum SongChockboxAction {
        Add(i64),
        Remove(i64),
    }

    struct SongsForSetlist {
        pub data: Vec<i64>,
    }

    impl Reducible for SongsForSetlist {
        type Action = SongChockboxAction;

        fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
            match action {
                SongChockboxAction::Add(value) => {
                    let mut list = self.data.clone();

                    list.push(value);

                    SongsForSetlist { data: list }.into()
                }
                SongChockboxAction::Remove(value) => {
                    let mut list = self.data.clone();
                    list.retain(|song_id| *song_id != value);
                    SongsForSetlist { data: list }.into()
                }
            }
        }
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
    let navigator = use_navigator().expect("There is no reason it will not work.");

    let on_setlist_submit = {
        let songs_for_setlist = songs_for_setlist.clone();

        Callback::from(move |e: SubmitEvent| {
            let navigator = navigator.clone();
            e.prevent_default();
            let form_data = FormData::new_with_form(&e.target_unchecked_into())
                .expect("This is going to work since we are sure form exists.");

            let new_setlist = NewSetlist {
                display_title: match form_data.get("display_title").as_string() {
                    Some(display_title) => display_title,
                    None => panic!(),
                },
                songs: (*songs_for_setlist.data).to_vec(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("http://127.0.0.1:8081/setlists")
                    .json(&new_setlist)
                    .expect("This will work")
                    .send()
                    .await;

                match response {
                    Ok(possible_setlist_id) => {
                        let setlist_id: Result<i64, _> = possible_setlist_id.json().await;
                        match setlist_id {
                            Ok(id) => navigator.push(&Route::Setlist { id }),
                            Err(_) => panic!(),
                        }
                    }
                    Err(_) => panic!(),
                };
            });
        })
    };

    html! {
    <>
        <article>
            <form onsubmit={ on_setlist_submit }>
                <fieldset disabled={ songs_for_setlist.data.is_empty() }>
                    <label for="display_title">{"Setlist name"}</label>
                    <input type="text" name="display_title" id="display_title"/>
                    <button type="submit">
                        {"Create setlist from checked songs"}
                    </button>
                </fieldset>
            </form>
        </article>
    <table>
        <caption>{ "Setlist" }</caption>
        <thead>
            <th>{ "Song" }</th>
            <th>{ "Cover" }</th>
            <th>{ "Chords" }</th>
        </thead>
        <tbody>
        {
            match &*setlist {
                Some(setlist) => {
                    html! {
                        for setlist.data.iter().map(|( song_id, song )|
                        html! {
                            <tr data-song_id={ format!("{}", song_id) }>
                                <td><input type="checkbox" name="song" onclick={ on_song_checkbox_click.clone() } value={ format!("{}", song_id) }/></td>
                                <td>{ song.name.clone() }</td>
                                <td>{ match &song.cover {
                                    Some(cover) => html! {<a href={ cover[0].url.clone() } target="_blank">{
                                        match cover[0].display_title.clone() {
                                            Some(display_title) => display_title,
                                            None => cover[0].url.clone(),
                                        }
                                    }</a>},
                                    None => "".into(),
                                    }
                                }</td>
                                <td>{ song.chords.clone() }</td>
                            </tr>
                        })
                    }
                },
                None => html! {
                    <tr>
                        <td>{"No data"}</td>
                    </tr>
                },
            }
        }
        </tbody>
    </table>
    </>
    }
}
