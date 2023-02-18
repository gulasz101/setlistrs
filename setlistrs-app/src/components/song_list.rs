use gloo_net::http::Request;

use setlistrs_types::SongList;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::SongChockboxAction;
use crate::components::SongsForSetlist;

#[function_component(SongsList)]
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
